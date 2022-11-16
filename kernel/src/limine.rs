// This module handles all things limine
// See more about the protocol: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md
use core::convert::TryFrom;
use core::ffi::CStr;
use core::iter::Iterator;
use lazy_static::lazy_static;
use spin::Mutex;

// first two items in .id of all requests must be equal to the following magic
// TODO: check this in init() for all requests
const MAGIC_COMMON: (u64, u64) = (0xc7b1dd30df4c8b88, 0x0a82e883a194f07b);

// simple pointer wrappers that can be replaced in the future for something like NonNull<T>
type Ptr<T> = *const T;
type MutPtr<T> = *mut T;

// See more: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-callback
type TerminalCallbackFunction = extern "C" fn(Ptr<Terminal>, u64, u64, u64, u64);

// function provided by limine to simplify display access
// WARNING: The function is NOT thread-safe, NOT reentrant, per-terminal.
// we access it from a Mutex<TerminalWriter> e.g. TERM0
type TerminalWriteFunction = extern "C" fn(Ptr<Terminal>, *const [u8], usize);

// Linked from kentry/limine.asm
extern "C" {
    static LIMINE_REQUEST_BOOT_INFO: RequestBootInfo;
    static LIMINE_REQUEST_TERMINAL: RequestTerminal;
    static LIMINE_REQUEST_MEMORY_MAP: RequestMemoryMap;
    static LIMINE_REQUEST_BOOT_TIME: RequestBootTime;
    static LIMINE_REQUEST_KERNEL_ADDRESS: RequestKernelAddress;
    static LIMINE_REQUEST_HHDM: RequestHHDM;
    static LIMINE_REQUEST_STACK_SIZE: RequestStackSize;
}

lazy_static! {
    // handles concurrent terminal.write() calls
    static ref TERM0: Mutex<TerminalWriter> =
        Mutex::new(TerminalWriter::new(0).expect("Could not open limine terminal"));
}

// public interface to print to TERM0
// accepts non ASCII (non utf8 strings) -> b"Hello"
pub fn print_bytes(s: &[u8]) {
    let access = TERM0.lock();
    ((access).write)(access.get_terminal(), s, s.len());
}

pub fn print_hex(mut n: usize) {
    let mut x: [u8; 18] = [0; 18];
    x[0] = b'0';
    x[1] = b'x';
    for i in 0..14 {
        let d = (n % 16) as u8;
        if d < 10 {
            x[17 - i] = d + 48;
        } else {
            x[17 - i] = d + 55;
        }
        n /= 16;
    }
    let access = TERM0.lock();
    ((access).write)(access.get_terminal(), &x, x.len());
}

pub fn print_dec(mut n: usize) {
    let mut x: [u8; 20] = [0; 20];
    for i in 0..x.len() {
        x[19 - i] = (n % 10 + 48) as u8;
        n /= 10;
    }
    let access = TERM0.lock();
    ((access).write)(access.get_terminal(), &x, x.len());
}

// ======= Boot Info feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-feature

// returns the bootloaders -> (name, version)
// WARNING: assumes the response has static lifetime
pub fn bootloader_info() -> (&'static [u8], &'static [u8]) {
    let response = unsafe { &*(LIMINE_REQUEST_BOOT_INFO.response) };
    (
        // SAFETY: ptr must not be null and must hold null terminated string
        unsafe { CStr::from_ptr(response.name as *const i8).to_bytes() },
        unsafe { CStr::from_ptr(response.version as *const i8).to_bytes() },
    )
}

#[repr(C)]
struct RequestBootInfo {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseBootInfo>,
}

#[repr(C)]
pub struct ResponseBootInfo {
    revision: u64,
    pub name: *const u8,
    pub version: *const u8,
}

// ======= Memory Map feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature

// private

#[repr(C)]
struct RequestMemoryMap {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseMemoryMap>,
}

#[derive(Clone)]
#[repr(C)]
struct ResponseMemoryMap {
    revision: u64,
    entry_count: u64,
    // has length of entry_count
    entries: Ptr<Ptr<MemoryMapEntry>>,
}

#[repr(C)]
struct MemoryMapEntry {
    base: u64,
    length: u64,
    typ: u64, // cast to MemmapEntryType
}

// public

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

// TODO: rewrite this using a macro or a derive
impl Into<&'static [u8]> for MemmapEntryType {
    fn into(self) -> &'static [u8] {
        match self {
            MemmapEntryType::Usable => b"Usable              ",
            MemmapEntryType::Reserved => b"Reserved            ",
            MemmapEntryType::AcpiReclaimable => b"ACPI Reclaimable    ",
            MemmapEntryType::AcpiNvs => b"ACPI NVS            ",
            MemmapEntryType::BadMemory => b"Bad Memory!         ",
            MemmapEntryType::BootloaderReclaimable => b"BL Reclaimable      ",
            MemmapEntryType::KernelAndModules => b"Kernel & Modules    ",
            MemmapEntryType::MemmapFramebuffer => b"Memmap Framebuffer  ",
        }
    }
}

// extern interface function used by the rest of the kernel
pub fn memory_map() -> MemoryMap {
    MemoryMap::new(unsafe { &*(LIMINE_REQUEST_MEMORY_MAP.response) })
}

// rust-friendly version of ResponseMemoryMap
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

// rust-friendly version of MemoryMapEntry
pub struct MemmapItem {
    pub range: (usize, usize),
    pub typ: MemmapEntryType,
}

// ======= Boot Time feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#boot-time-feature

#[repr(C)]
struct RequestBootTime {
    id: [u64; 4],
    revision: u64,
    response: *const ResponseBootTime,
}

#[repr(C)]
struct ResponseBootTime {
    revision: u64,
    time: i64,
}

pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

// parses the unix time stamp into a more readable structure (Time)
pub fn boot_time() -> Time {
    todo!()
}

pub fn boot_time_stamp() -> i64 {
    unsafe { (*LIMINE_REQUEST_BOOT_TIME.response).time }
}

// ======= Kernel Address feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#kernel-address-feature

#[repr(C)]
struct RequestKernelAddress {
    id: [u64; 4],
    revision: u64,
    response: *const ResponseKernelAddress,
}

#[repr(C)]
struct ResponseKernelAddress {
    revision: u64,
    physical_base: u64,
    virtual_base: u64,
}

pub fn kernel_address_physical() -> usize {
    (unsafe { (*LIMINE_REQUEST_KERNEL_ADDRESS.response).physical_base }) as usize
}

pub fn kernel_address_virtual() -> usize {
    (unsafe { (*LIMINE_REQUEST_KERNEL_ADDRESS.response).virtual_base }) as usize
}

// ======= HHDM (higher half direct map) feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#hhdm-higher-half-direct-map-feature

#[repr(C)]
struct RequestHHDM {
    id: [u64; 4],
    revision: u64,
    response: *const ResponseHHDM,
}

#[repr(C)]
struct ResponseHHDM {
    revision: u64,
    offset: u64, // virtual address of the beginning of the HHDM (higher half direct map)
}

pub fn hhdm() -> usize {
    (unsafe { (*LIMINE_REQUEST_HHDM.response).offset }) as usize
}

// ======= Stack Size feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#stack-size-feature

#[repr(C)]
struct RequestStackSize {
    id: [u64; 4],
    revision: u64,
    response: *const ResponseStackSize,
    size: u64,
}

#[repr(C)]
struct ResponseStackSize {
    revision: u64,
}

// ======= Terminal feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#bootloader-info-feature

struct TerminalWriter {
    term: usize, // pointer to terminal
    write: TerminalWriteFunction,
}

// handles
impl TerminalWriter {
    fn new(terminal_number: u64) -> Option<Self> {
        let term_resp = unsafe { &*(LIMINE_REQUEST_TERMINAL.response) };
        if term_resp.terminal_count > terminal_number {
            return Some(Self {
                term: term_resp.terminals as usize + terminal_number as usize,
                write: term_resp.write,
            });
        }
        None
    }

    fn get_terminal(&self) -> *const Terminal {
        self.term as *const Terminal
    }

    // exposes the (width, height) of term
    fn dimensions(&self) -> (u64, u64) {
        let t = unsafe { &*self.get_terminal() };
        (t.columns, t.rows)
    }
}

#[repr(C)]
struct RequestTerminal {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseTerminal>,
    callback: TerminalCallbackFunction,
}

#[repr(C)]
struct ResponseTerminal {
    revision: u64,
    terminal_count: u64,
    terminals: Ptr<Ptr<Terminal>>,
    write: TerminalWriteFunction,
}

#[repr(C)]
struct Terminal {
    columns: u64,
    rows: u64,
    framebuffer: Ptr<Framebuffer>,
}

// use by the Terminal feature and the Framebuffer feature
#[repr(C)]
struct Framebuffer {
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
