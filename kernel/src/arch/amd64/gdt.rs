/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// loads the Global Descriptor Table
// main source: https://wiki.osdev.org/Global_Descriptor_Table

use x86::dtables::{lgdt, sgdt, DescriptorTablePointer};
#[macro_use]
use crate::tools::{bin_extract, bin_insert};

const PRIVILEGE_KERNEL: u8 = 0;
const PRIVILEGE_USER: u8 = 3;

static GDT: &[SegmentDescriptor] = &[
    // this exact structure must be preserved for limine facilities to work
    SegmentDescriptor::null(),
    SegmentDescriptor::new_kernel_code16(),
    SegmentDescriptor::new_kernel_data16(),
    SegmentDescriptor::new_kernel_code32(),
    SegmentDescriptor::new_kernel_data32(),
    SegmentDescriptor::new_kernel_code64(),
    SegmentDescriptor::new_kernel_data64(),
    // after this anything can be loaded
];

/* const_bitfield implementation of SegmentDescriptor

use const_bitfield::bitfield;
bitfield! {
    #[derive(Debug)]
    struct SegmentDescriptor(u64);
    u16, limit0, set_limit0: 15, 0;
    u16, base0,  set_base0: 31, 16;
    u8,  base1,  set_base1: 39, 32;

    // access byte 40-47
    bool, access_A, set_access_a: 40; // Accessed
    bool, access_RW, set_access_rw: 41; // Readable/Writable
    bool, access_DC, set_access_dc: 42; // Director/Conforming
    bool, access_E, set_access_e: 43;  // executable
    bool, access_S, set_access_s: 44;   // type
    u8, access_DPL, set_access_dpl: 46, 45; // descriptor prviliege level
    bool, access_P, set_access_p: 47; // present

    u8, limit1, set_limit1: 51, 48;

    // flags 52-55
    bool, reserved, _: 52;
    bool, flag_L, set_flag_l: 53; // Long-mode
    bool, flag_DB, set_flag_db: 54; // size
    bool, flag_G, set_flag_g: 55; // granularity

    u8, base2, set_base2: 63, 56;
}
*/

struct SegmentDescriptor(u64);

macro_rules! bitfield {
    // setters
    ($name:ident, $typ:ty, $h:literal, $l:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $h, $l));
        }
    };

    ($name:ident, $typ:ty, $b:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $b, $b));
        }
    };
}

impl SegmentDescriptor {
    // bit write & readers
    bitfield!(set_limit0, u16, 15, 0);
    bitfield!(set_base0, u16, 31, 16);
    bitfield!(set_base1, u8, 39, 32);

    bitfield!(set_access_a, bool, 40);
    bitfield!(set_access_rw, bool, 41);
    bitfield!(set_access_dc, bool, 42);
    bitfield!(set_access_e, bool, 43);
    bitfield!(set_access_s, bool, 44);
    bitfield!(set_access_dpl, u8, 46, 45);
    bitfield!(set_access_p, bool, 47);

    bitfield!(set_limit1, u8, 51, 48);

    bitfield!(set_reserved, bool, 52); // dont use this
    bitfield!(set_flag_l, bool, 53);
    bitfield!(set_flag_db, bool, 54);
    bitfield!(set_flag_g, bool, 55);

    bitfield!(set_base2, u8, 63, 56);

    const fn null() -> Self {
        Self(0_u64)
    }

    // addr: u20
    const fn set_whole_limit(&mut self, addr: u32) {
        self.set_limit0(bin_extract(addr, 15, 0) as u16);
        self.set_limit1(bin_extract(addr, 19, 16) as u8);
    }

    const fn set_whole_base(&mut self, addr: u32) {
        self.set_base0(bin_extract(addr, 15, 0) as u16);
        self.set_base1(bin_extract(addr, 23, 16) as u8);
        self.set_base2(bin_extract(addr, 31, 24) as u8);
    }

    // base constructor for kernel descriptors
    const fn new_kernel() -> Self {
        let mut sd = Self::null();
        sd.set_access_p(true); // present
        sd.set_access_dpl(PRIVILEGE_KERNEL);
        sd.set_access_s(true); // non-system type -> code / data
        sd.set_access_dc(false); // non conforming to lower rings
        sd.set_access_a(false); // managed by the cpu, left null
        sd.set_flag_l(false); // true only for 64 bit code descriptors
        sd.set_access_rw(true); // read/write enabled for code/data descriptors
        sd.set_whole_base(0); // all default segments start at 0x0
        return sd;
    }

    const fn new_kernel_code16() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_whole_limit(0xFFFF);
        sd.set_access_e(true); // executable
        sd.set_flag_g(false);
        sd.set_flag_db(false); // 16 bit protected mode
        return sd;
    }

    const fn new_kernel_data16() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_whole_limit(0xFFFF);
        sd.set_access_e(false);
        sd.set_flag_g(false);
        sd.set_flag_db(false);
        // magic
        sd.set_access_a(true);
        return sd;
    }

    const fn new_kernel_code32() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_whole_limit(1048575);
        sd.set_access_e(true);        
        sd.set_flag_g(true);
        sd.set_flag_db(true);
        return sd;
    }

    const fn new_kernel_data32() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_whole_limit(1048575);
        sd.set_access_e(false);
        sd.set_flag_g(true);
        sd.set_flag_db(true);
        // magic
        sd.set_access_a(true);
        return sd;
    }

    // limitx & base& are ignored in 64 bit mode because they cover the whole address space
    const fn new_kernel_code64() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_access_e(true); // executable
        sd.set_access_rw(true); // allow read, exec enabled by default
        sd.set_flag_g(false);
        sd.set_flag_db(false); // clear since flag_L is enabled
        sd.set_flag_l(true); // long mode 64 bit
        return sd;
    }

    const fn new_kernel_data64() -> Self {
        let mut sd = Self::new_kernel();
        sd.set_access_e(false); // non-executable (data)
        sd.set_access_rw(true); // allow write, read enabled by default
        sd.set_flag_g(false);
        sd.set_flag_db(false);
        sd.set_flag_l(false); // non-64 bit executable -> data
        sd.set_access_a(true);
        return sd;
    }
}

pub fn init() {
    let mut loaded: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer::default();
    let gdt = DescriptorTablePointer::new_from_slice(GDT);
    unsafe { lgdt(&gdt) };
}
