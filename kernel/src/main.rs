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
    loop {}
}

mod arch;
mod limine;

#[no_mangle]
pub extern "C" fn kmain() {
    limine::print0(b"Hello Kernel World!\n");

    let (blname, blversion) = limine::bootloader_info();
    limine::print0(b"[Detected bootloader info]\n");
    limine::print0(b"--> name: ");
    limine::print0(blname);
    limine::print0(b"\n--> version: ");
    limine::print0(blversion);
    limine::print0(b"\n");
    loop {}
}
