#![no_std]
#![no_main]
#![crate_type = "staticlib"]
#![feature(core_c_str)]
#![feature(layout_for_ptr)]

use core::panic::{self, PanicInfo};
// Do not remove these imports, they prevent link errors
#[allow(unused_imports)]
pub use rlibc;
pub use rlibcex;

#[panic_handler]
fn kpanic(_pi: &core::panic::PanicInfo<'_>) -> ! {
    log!("KERNEL PANIC!!!\n");
    loop {}
}

mod arch;
mod limine;
mod log;
mod memman;
use memman::map::MemoryMapper; 

#[no_mangle]
pub extern "C" fn kmain() {
    log!("Hello World!\n");

    // hardware
    let boot_time = limine::boot_time_stamp();
    log!("[ Hardware Info ]\n");
    log!("UNIX Boot time: {}\n", boot_time);

    // arch
    log!("CPU Architecture: ");
    match arch::get_arch() {
        arch::ArchType::X86_64 => log!("x86_64\n"),
        arch::ArchType::Arm64 => log!("Arm64/AArch64\n"),
    };

    // boot loader
    let (bootloader_name, bootloader_version) = limine::bootloader_info();
    log!("[ Bootloader info ]\n");
    log!("name: {}\n", core::str::from_utf8(bootloader_name).unwrap());
    log!("version: {}\n", core::str::from_utf8(bootloader_version).unwrap());

    // memory map
    log!("[ Memory Map ]\n");

    let ram_size = limine::memory_map().last().unwrap().range.1;
    unsafe { memman::map::set_global((0, ram_size)) };

    for region in limine::memory_map() {
        let (start, end) = region.range;
        match region.typ {
            limine::MemmapEntryType::Usable => {},
            _ => { memman::map::claim_global(region.range).unwrap(); }
        }
        let typ: &str = region.typ.into();
        log!("{:023} 0x{:016X} - 0x{:016X}\n", typ, start, end);
    }

    /*
    // log GLOBAL_MEMORY_MAPPER entries
    use memman::map::MemoryMapper;
    for a in memman::map::GLOBAL_MEMORY_MAPPER.get().unwrap().iter() {
        limine::print_hex(a.0);
        log!(b"\n");
    }
    */

    // kernel address
    let kernel_physical_address = limine::kernel_address_physical();
    let kernel_virtual_address = limine::kernel_address_virtual();
    log!("[ Kernel Address ]\n");
    log!("physical: {:016X}\n", kernel_physical_address);
    log!("virtual:  {:016X}\n", kernel_virtual_address);

    // HHDM
    let hhdm = limine::hhdm();
    log!("HHDM: {:016X}\n", hhdm);

    log!("Nothing to do!\n");
    panic!("Nothing to do!");
}
