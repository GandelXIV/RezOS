#![no_std]
#![no_main]
#![crate_type = "staticlib"]

use core::panic::{self, PanicInfo};
use rlibc;


#[panic_handler]
fn kpanic(pi: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { arch::cpu::halt(); }
    loop {}
}

mod bootboot;
mod arch;
mod io;

use bootboot::*;
use io::serial;

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

// regular .unwrap() triple faults for some reason, so use this tmp

fn unwrap_opt<T>(opt: Option<T>) -> T {
    match opt {
        Some(data) => data,
        None => panic!(),
    }
}

fn unwrap_res<T, U>(res: Result<T, U>) -> T {
    match res {
        Ok(data) => data,
        Err(e) => panic!(),
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // bootboot init
    let bootboot = &unsafe { *(BOOTBOOT_INFO as *const BOOTBOOT_HEADER) };
    if !slicecmp(&bootboot.magic, BOOTBOOT_MAGIC) {
        panic!()
    }

    // arch init
    arch::init();

    // serial init
    let mut log;
    match serial::init(bootboot) {
        Ok(_) => {
            log = unwrap_res(serial::access(1));
            log.write_str("Hello kernel world!\n");
        }
        Err(e) => panic!(),
    }

    loop {}
}
