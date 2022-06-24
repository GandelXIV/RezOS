#![no_std]
#![no_main]

use core::panic::{self, PanicInfo};
use rlibc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod bootboot;
use bootboot::*;

fn slicecmp<T: core::cmp::PartialEq>(x: &[T], y: &[T]) -> bool {
    if x.len() != y.len() {
        return false;
    }
    for i in 0..x.len() {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn kmain() {
    // init
    let bootboot = &unsafe { *(BOOTBOOT_INFO as *const BOOTBOOT) };
    if !slicecmp(&bootboot.magic, BOOTBOOT_MAGIC) {
        // is magic valid?
        // invalid magic
        loop {}
    }

    // draw white rect
    let fb: usize = 0xFFFFFFFFFC000000;
    let bar = 3200;
    for x in 0..bar * 100 {
        let pixel = (fb + x) as *mut u32;
        unsafe { *pixel = 0x00FFFFFF }
    }
    loop {}
}
