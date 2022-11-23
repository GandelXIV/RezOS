// loads the Global Descriptor Table
// main source: https://wiki.osdev.org/Global_Descriptor_Table

use const_bitfield::bitfield;
use x86::dtables::{lgdt, DescriptorTablePointer};

static GDT: &[SegmentDescriptor] = &[
    SegmentDescriptor::null(),
    SegmentDescriptor::new_kernel_code(),
    SegmentDescriptor::new_kernel_data()
];

bitfield! {
    struct SegmentDescriptor(u64);
    // limitx & basex are ignored in 64 bit mode
    u16, limit0, _: 15, 0;
    u16, base0,  _: 31, 16;
    u8,  base1,  _: 39, 32;
    
    // access byte
    bool, access_A, set_access_A: 40;
    bool, access_RW, set_access_RW: 41;
    bool, access_DC, set_access_DC: 42;
    bool, access_E, set_access_E: 43;
    bool, access_S, set_access_S: 44;
    u8, access_DPL, set_access_DPL: 46, 45;
    bool, access_present, set_access_present: 47;
    u8, limit1, _: 51, 48;

    // flags
    bool, reserved, _: 52;
    bool, flag_L, set_flag_L: 53;
    bool, flag_DB, set_flag_DB: 54;
    bool, flag_G, set_flag_G: 55;

    u8, base2, _: 63, 56;
}

impl SegmentDescriptor {
    const fn null() -> Self {
        Self(0_u64)
    }

    const fn new_kernel_code() -> Self {
        todo!()
    }

    const fn new_kernel_data() -> Self {
        todo!()
    }
}

pub fn init() {
    unsafe { x86::dtables::lgdt(&DescriptorTablePointer::new_from_slice(GDT)) };
}
