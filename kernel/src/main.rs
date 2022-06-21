#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() {
    let fb: usize = 0xFFFFFFFFFC000000;
    let bar = 3200;
    for x in 0..bar * 100 {
        let pixel = (fb + x) as *mut u32;
        unsafe {
            *pixel = 0x00FFFFFF;
        }
    }
    loop {}
}
