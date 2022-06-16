#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain() {
    let vga = 0xB8000 as *mut char;
    unsafe {
        *vga = 'X';
    }
    loop {}
}