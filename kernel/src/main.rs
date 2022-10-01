#![no_std]
#![no_main]
#![crate_type = "staticlib"]

// Do not remove this import, it prevents link errors
use rlibc;
use core::panic::{self, PanicInfo};

#[panic_handler]
fn kpanic(_pi: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() {
    loop {}
}
