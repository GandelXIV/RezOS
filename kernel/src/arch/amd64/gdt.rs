// loads the Global Descriptor Table
// main source: https://wiki.osdev.org/Global_Descriptor_Table

// TODO: This priple faults because the limine.write() function requires a specific GDT order to be
// setup: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#x86_64-1

use const_bitfield::bitfield;
use x86::dtables::{lgdt, DescriptorTablePointer};
#[macro_use]
use crate::log;

const PRIVILEGE_KERNEL: u8 = 0;
const PRIVILEGE_USER: u8 = 3;

static GDT: &[SegmentDescriptor] = &[
    SegmentDescriptor::null(),
];

bitfield! {
    struct SegmentDescriptor(u64);
    u16, limit0, set_limit0: 15, 0;
    u16, base0,  set_base0: 31, 16;
    u8,  base1,  set_base1: 39, 32;

    // access byte 40-47
    bool, access_A, set_access_A: 40; // Accessed
    bool, access_RW, set_access_RW: 41; // Readable/Writable
    bool, access_DC, set_access_DC: 42; // Director/Conforming
    bool, access_E, set_access_E: 43;  // executable
    bool, access_S, set_access_S: 44;   // type
    u8, access_DPL, set_access_DPL: 46, 45; // descriptor prviliege level
    bool, access_P, set_access_P: 47; // present

    u8, limit1, set_limit1: 51, 48;

    // flags 52-55
    bool, reserved, _: 52;
    bool, flag_L, set_flag_L: 53; // Long-mode
    bool, flag_DB, set_flag_DB: 54; // size
    bool, flag_G, set_flag_G: 55; // granularity

    u8, base2, set_base2: 63, 56;
}

impl SegmentDescriptor {
    const fn null() -> Self {
        Self(0_u64)
    }
    
    // addr: u20
    fn set_whole_limit(&mut self, addr: u32) {
        todo!()
    }

    fn set_whole_base(&mut self, addr: u32) {
        todo!()
    }

    // limitx & base& are ignored in 64 bit mode
    const fn new_kernel_code16() -> Self {
        todo!()
    }

    const fn new_kernel_code64() -> Self {
        let mut sd = Self::null();
        sd.set_access_P(true); // enabled
        sd.set_access_DPL(PRIVILEGE_KERNEL); // kernel runs in ring 0
        sd.set_access_S(true); // non-system because it holds code
        sd.set_access_E(true); // executable
        sd.set_access_DC(false); // non-conforming to lower ring levels
        sd.set_access_RW(true); // allow read
        sd.set_access_A(false); // let 0, the CPU will manage it
        sd.set_flag_G(true); // 4KiB granularity (same as page)
        sd.set_flag_DB(false); // clear since flag_L is enabled
        sd.set_flag_L(true); // long mode 64 bit
        return sd;
    }

    const fn new_kernel_data64() -> Self {
        let mut sd = Self::null();
        sd.set_access_P(true); // enabled
        sd.set_access_DPL(PRIVILEGE_KERNEL); // kernel runs in ring 0
        sd.set_access_S(true); // non-system because it holds data
        sd.set_access_E(false); // non-executable (data)
        sd.set_access_DC(false); // non-conforming to lower ring levels
        sd.set_access_RW(true); // allow write, read enabled by default
        sd.set_access_A(false); // let 0, the CPU will manage it
        sd.set_flag_G(true); // 4KiB granularity (same as page)
        sd.set_flag_DB(true); // 32 bit address space
        sd.set_flag_L(false); // non-64 bit executable -> data
        return sd;
    }
}

pub fn init() {
    unsafe { x86::dtables::lgdt(&DescriptorTablePointer::new_from_slice(GDT)) };
    loop {}
}
