#![no_std]
#![no_main]
#![crate_type = "staticlib"]

use core::panic::{self, PanicInfo};
#[allow(unused_imports)]
// Do not remove this import, it prevents link errors
use rlibc;

#[panic_handler]
fn kpanic(_pi: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

mod arch;
mod limine;

#[no_mangle]
pub extern "C" fn kmain() {
    arch::init();
    limine::init();
    loop {}
}
