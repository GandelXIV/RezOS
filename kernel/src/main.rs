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
    limine::print_bytes(b"\nKERNEL PANIC!!!\n");
    loop {}
}

mod arch;
mod limine;
mod memman;

#[no_mangle]
pub extern "C" fn kmain() {
    limine::print_bytes(b"Hello World!\n");
    init_limine();
    limine::print_bytes(b"\nNothing to do!\n");
    loop {}
}

fn init_limine() {
    // hardware
    limine::print_bytes(b"[ Hardware Info ]\n");
    limine::print_bytes(b"UNIX Boot time: ");
    limine::print_dec(limine::boot_time_stamp() as usize);
    limine::print_bytes(b"\n");
    // arch
    limine::print_bytes(b"CPU Architecture: ");
    match arch::get_arch() {
        arch::ArchType::X86_64 => limine::print_bytes(b"x86_64"),
        arch::ArchType::Arm64 => limine::print_bytes(b"Arm64/AArch64"),
    };
    // boot loader
    let (blname, blversion) = limine::bootloader_info();
    limine::print_bytes(b"\n[ Bootloader info ]\n");
    limine::print_bytes(b"name: ");
    limine::print_bytes(blname);
    limine::print_bytes(b"\nversion: ");
    limine::print_bytes(blversion);
    limine::print_bytes(b"\n");
    // memory map
    limine::print_bytes(b"[ Memory Map ]\n");
    for region in limine::memory_map() {
        let (start, end) = region.range;
        limine::print_bytes(region.typ.into());
        limine::print_hex(start);
        limine::print_bytes(b" - ");
        limine::print_hex(end);
        limine::print_bytes(b"\n");
    }
    // kernel address
    limine::print_bytes(b"[ Kernel Address ]\n");
    limine::print_bytes(b"physical:  ");
    limine::print_hex(limine::kernel_address_physical());
    limine::print_bytes(b"\nvirtual:   ");
    limine::print_hex(limine::kernel_address_virtual());
    limine::print_bytes(b"\n");
}
