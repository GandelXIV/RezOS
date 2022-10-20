#![no_std]
#![no_main]
#![crate_type = "staticlib"]
#![feature(core_ffi_c)]

use core::panic::{self, PanicInfo};
use core::fmt::write;
#[allow(unused_imports)]
// Do not remove these imports, they prevent link errors
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
    limine::print0("Hello Kernel World!\n");
    loop {}
}
