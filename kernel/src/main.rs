#![no_std]
#![no_main]
#![crate_type = "staticlib"]

use core::panic::{self, PanicInfo};
use core::fmt::write;
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
    loop {}
}
