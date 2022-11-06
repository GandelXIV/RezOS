#![no_std]
#![no_main]
#![crate_type = "staticlib"]
#![feature(core_c_str)]

use core::panic::{self, PanicInfo};
// Do not remove these imports, they prevent link errors
#[allow(unused_imports)]
pub use rlibc;
pub use rlibcex;

#[panic_handler]
fn kpanic(_pi: &core::panic::PanicInfo<'_>) -> ! {
    limine::print0(b"\nKERNEL PANIC!!!\n");
    loop {}
}

mod arch;
mod limine;

#[no_mangle]
pub extern "C" fn kmain() {
    limine::print0(b"Hello World!\n");
    // arch 
    limine::print0(b"CPU Architecture: ");
    match arch::get_arch() {
        arch::ArchType::X86_64 => limine::print0(b"x86_64"),
        arch::ArchType::Arm64 => limine::print0(b"Arm64/AArch64"),
    };
    // boot loader
    let (blname, blversion) = limine::bootloader_info();
    limine::print0(b"\n[Bootloader info]\n");
    limine::print0(b"--> name: ");
    limine::print0(blname);
    limine::print0(b"\n--> version: ");
    limine::print0(blversion);
    limine::print0(b"\n");
    // memory map
    limine::print0(b"[Memory Map]\n");
    let mm = limine::memory_map();
    // TODO: print region area
    for region in mm {
        limine::print0(region.typ.into());
        limine::print0(b" (X - X)\n");
    }
    
    limine::print0(b"Nothing to do!\n");
    loop {}
}
