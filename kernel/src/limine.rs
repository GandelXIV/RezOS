/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//!
//! <br> See more about the protocol: `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md`

use embedded_graphics::pixelcolor::raw::RawU32;
use embedded_graphics::pixelcolor::{Rgb555, Rgb565, Rgb888};
use embedded_graphics::prelude::*;

use crate::enum_names;
use core::convert::TryFrom;
use core::ffi::CStr;
use core::iter::Iterator;

/// simple pointer wrapper that can be replaced in the future for something like `NonNull<T>`
type Ptr<T> = *const T;
/// simple mutable pointer wrapper that can be replaced in the future for something like `NonNull<T>`
type MutPtr<T> = *mut T;

// Linked from kentry/limine.asm
extern "C" {
    static LIMINE_REQUEST_BOOT_INFO: RequestBootInfo;
    static LIMINE_REQUEST_MEMORY_MAP: RequestMemoryMap;
    static LIMINE_REQUEST_BOOT_TIME: RequestBootTime;
    static LIMINE_REQUEST_KERNEL_ADDRESS: RequestKernelAddress;
    static LIMINE_REQUEST_HHDM: RequestHHDM;
    static LIMINE_REQUEST_STACK_SIZE: RequestStackSize;
    static LIMINE_REQUEST_FRAMEBUFFER: RequestFrameBuffer;
}

/*
/// public interface to print to TERM0
/// , accepts ASCII (non utf8 strings) e.g. `b"Hello"`
///
/// It is not adviced to use this, as the bootloader facilities may be reclaimed.
pub fn print_bytes(s: &[u8]) {
    let access = TERM0.lock();
    ((access).write)(access.get_terminal(), s, s.len());
}
*/

/// macro that completes a request and response struct with all the default fields.
///
/// The workings of this macro can be deduced from the source code or context. <br>
/// See more about limine features: `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#features`
macro_rules! limine_feature {
    (
        #[doc = $doc:expr]
        struct $request:ident {
            $($req_field_key:ident : $req_field_type:ty,)*
        }

        struct $response:ident {
            $($res_field_key:ident : $res_field_type:ty,)*
        }
    ) => {
        #[repr(C)]
        #[doc = $doc]
        struct $request {
            id: [u64; 4],
            revision: u64,
            response: Ptr<$response>,
            $($req_field_key : $req_field_type,)*
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc = $doc]
        struct $response {
            revision: u64,
            $($res_field_key : $res_field_type,)*
        }
    };
}

// ======= Boot Info feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-feature

/// returns the bootloaders name and version
///
/// WARNING: assumes the response has static lifetime
pub fn bootloader_info() -> (&'static [u8], &'static [u8]) {
    let response = unsafe { &*(LIMINE_REQUEST_BOOT_INFO.response) };
    (
        // SAFETY: ptr must not be null and must hold null terminated string
        unsafe { CStr::from_ptr(response.name as *const i8).to_bytes() },
        unsafe { CStr::from_ptr(response.version as *const i8).to_bytes() },
    )
}

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-feature`

    struct RequestBootInfo{}

    struct ResponseBootInfo {
        name: *const u8,
        version: *const u8,
    }
}

// ======= Memory Map feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature

// private

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature`

    struct RequestMemoryMap {}

    struct ResponseMemoryMap {
        entry_count: u64,
        // has length of entry_count
        entries: Ptr<Ptr<MemoryMapEntry>>,
    }
}

/// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature`
#[repr(C)]
struct MemoryMapEntry {
    base: u64,
    length: u64,
    typ: u64, // cast to MemmapEntryType
}

// public

// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature`
// Implements `Into<&str>` for its enum variant names
enum_names! {
    pub enum MemmapEntryType {
        Usable,
        Reserved,
        AcpiReclaimable,
        AcpiNvs,
        BadMemory,
        BootloaderReclaimable,
        KernelAndModules,
        MemmapFramebuffer,
    }
}

// described in the limine protocol specs
impl TryFrom<u64> for MemmapEntryType {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Usable),
            1 => Ok(Self::Reserved),
            2 => Ok(Self::AcpiReclaimable),
            3 => Ok(Self::AcpiNvs),
            4 => Ok(Self::BadMemory),
            5 => Ok(Self::BootloaderReclaimable),
            6 => Ok(Self::KernelAndModules),
            7 => Ok(Self::MemmapFramebuffer),
            _ => Err(()),
        }
    }
}

/// extern interface function used by the rest of the kernel
pub fn memory_map() -> MemoryMap {
    MemoryMap::new(unsafe { &*(LIMINE_REQUEST_MEMORY_MAP.response) })
}

/// rust-friendly version of `ResponseMemoryMap`
pub struct MemoryMap {
    icount: u64,
    resp_copy: ResponseMemoryMap,
}

impl MemoryMap {
    fn new(base: &ResponseMemoryMap) -> Self {
        Self {
            icount: 0,
            resp_copy: base.clone(),
        }
    }
}

impl Iterator for MemoryMap {
    type Item = MemmapItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.icount >= self.resp_copy.entry_count {
            return None;
        }

        let e = unsafe { &*((*(self.resp_copy.entries)).offset(self.icount as isize)) };

        self.icount += 1;
        Some(Self::Item {
            range: (e.base as usize, (e.length + e.base) as usize),
            typ: MemmapEntryType::try_from(e.typ).unwrap(),
        })
    }
}

/// rust-friendly version of `MemoryMapEntry`
pub struct MemmapItem {
    pub range: (usize, usize),
    pub typ: MemmapEntryType,
}

// ======= Boot Time feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#boot-time-feature

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#boot-time-feature`

    struct RequestBootTime{}

    struct ResponseBootTime {
        time: i64,
    }
}

/// Gets the unix time at boot
pub fn boot_time_stamp() -> i64 {
    unsafe { (*LIMINE_REQUEST_BOOT_TIME.response).time }
}

// ======= Kernel Address feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#kernel-address-feature

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#kernel-address-feature`

    struct RequestKernelAddress {}

    struct ResponseKernelAddress {
        physical_base: u64,
        virtual_base: u64,
    }
}

/// Get the physical base address for the kernel
pub fn kernel_address_physical() -> usize {
    (unsafe { (*LIMINE_REQUEST_KERNEL_ADDRESS.response).physical_base }) as usize
}

/// Get the virtual base address for the kernel
pub fn kernel_address_virtual() -> usize {
    (unsafe { (*LIMINE_REQUEST_KERNEL_ADDRESS.response).virtual_base }) as usize
}

// ======= HHDM (higher half direct map) feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#hhdm-higher-half-direct-map-feature

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#hhdm-higher-half-direct-map-feature`

    struct RequestHHDM {}

    struct ResponseHHDM {
        offset: u64,
    }
}

/// Get the higher half direct map
pub fn hhdm() -> usize {
    (unsafe { (*LIMINE_REQUEST_HHDM.response).offset }) as usize
}

// ======= Stack Size feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#stack-size-feature

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#stack-size-feature`

    struct RequestStackSize {
        size: u64,
    }

    struct ResponseStackSize {}
}

// ======= Framebuffer feature
// See: https://github.com/limine-bootloader/limine/blob/v8.x/PROTOCOL.md#framebuffer-feature

limine_feature! {

    /// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#stack-size-feature`

    struct RequestFrameBuffer {}

    struct ResponseFrameBuffer {
        framebuffer_count: u64,
        framebuffers: MutPtr<MutPtr<Framebuffer>>,
    }
}

/// used by both the Terminal feature and the Framebuffer feature
///
/// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#framebuffer-feature`
/// `https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#bootloader-info-feature`
#[repr(C)]
#[derive(Debug)]
pub struct Framebuffer {
    pub address: MutPtr<u8>,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub reserved: [u8; 7],
    pub edid_size: u64,
    pub edid: Ptr<u8>,
}

pub fn framebuffer0() -> &'static mut Framebuffer {
    unsafe { &mut **(*LIMINE_REQUEST_FRAMEBUFFER.response).framebuffers }
}

#[derive(PartialEq, Copy, Clone)]
pub struct LColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Rgb555> for LColor {
    fn from(value: Rgb555) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
        }
    }
}

impl From<Rgb565> for LColor {
    fn from(value: Rgb565) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
        }
    }
}

impl From<Rgb888> for LColor {
    fn from(value: Rgb888) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
        }
    }
}

impl PixelColor for LColor {
    type Raw = RawU32;
}

impl OriginDimensions for Framebuffer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for Framebuffer {
    type Color = LColor;
    type Error = DrawPixelError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            draw_pixel(
                &self,
                coord.x as usize,
                coord.y as usize,
                (color.r, color.g, color.b),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum DrawPixelError {
    InvalidX,
    InvalidY,
    BadProperties,
}

pub fn draw_pixel(
    fb: &Framebuffer,
    x: usize,
    y: usize,
    color: (u8, u8, u8),
) -> Result<(), DrawPixelError> {
    if x > fb.width as usize {
        return Err(DrawPixelError::InvalidX);
    }

    if y > fb.height as usize {
        return Err(DrawPixelError::InvalidY);
    }

    if fb.bpp != 32 {
        return Err(DrawPixelError::BadProperties);
    }

    let (red, green, blue) = color;
    let pixc: u32 = ((red as u32) << fb.red_mask_shift)
        | ((green as u32) << fb.green_mask_shift)
        | ((blue as u32) << fb.blue_mask_shift);

    unsafe {
        *(fb.address as *mut u32)
            .wrapping_add(x)
            .wrapping_add(fb.width as usize * y) = pixc;
    }

    Ok(())
}
