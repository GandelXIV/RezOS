#![no_std]
#![no_main]
#![crate_type = "staticlib"]

// Do not remove this import, it prevents link errors
use core::panic::{self, PanicInfo};
#[allow(unused_imports)]
use rlibc;

#[panic_handler]
fn kpanic(_pi: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

// TODO: process these requests
extern "C" {
    static LIMINE_REQUEST_BOOT_INFO: LimineRequestBootInfo;
    static LIMINE_REQUEST_TERMINAL: LimineRequestBootInfo;
}



#[no_mangle]
pub extern "C" fn kmain() {
    loop {}
}
