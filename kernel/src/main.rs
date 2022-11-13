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

    // hardware
    limine::print_bytes(b"[ Hardware Info ]\n");
    limine::print_bytes(b"UNIX Boot time: ");
    let boot_time = limine::boot_time_stamp();
    limine::print_dec(boot_time as usize);
    limine::print_bytes(b"\n");

    // arch
    limine::print_bytes(b"CPU Architecture: ");
    match arch::get_arch() {
        arch::ArchType::X86_64 => limine::print_bytes(b"x86_64"),
        arch::ArchType::Arm64 => limine::print_bytes(b"Arm64/AArch64"),
    };

    // boot loader
    let (bootloader_name, bootloader_version) = limine::bootloader_info();
    limine::print_bytes(b"\n[ Bootloader info ]\n");
    limine::print_bytes(b"name: ");
    limine::print_bytes(bootloader_name);
    limine::print_bytes(b"\nversion: ");
    limine::print_bytes(bootloader_version);
    limine::print_bytes(b"\n");

    // memory map
    limine::print_bytes(b"[ Memory Map ]\n");

    let ram_size = limine::memory_map().last().unwrap().range.1;
    unsafe { memman::map::set_global((0, ram_size)) };

    for region in limine::memory_map() {
        let (start, end) = region.range;
        memman::map::claim_global(region.range);
        limine::print_bytes(region.typ.into());
        limine::print_hex(start);
        limine::print_bytes(b" - ");
        limine::print_hex(end);
        limine::print_bytes(b"\n");
    }

    // kernel address
    limine::print_bytes(b"[ Kernel Address ]\n");
    limine::print_bytes(b"physical:  ");
    let kernel_physical_address = limine::kernel_address_physical();
    limine::print_hex(kernel_physical_address);
    let kernel_virtual_address = limine::kernel_address_virtual();
    limine::print_bytes(b"\nvirtual:   ");
    limine::print_hex(kernel_virtual_address);
    limine::print_bytes(b"\n");
    // HHDM
    limine::print_bytes(b"HHDM: ");
    let hhdm = limine::hhdm();
    limine::print_hex(hhdm);
    limine::print_bytes(b"\n");

    limine::print_bytes(b"\nNothing to do!\n");
    loop {}
}
