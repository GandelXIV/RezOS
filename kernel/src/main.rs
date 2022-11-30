#![no_std]
#![no_main]
#![crate_type = "staticlib"]
#![feature(core_c_str)]
#![feature(layout_for_ptr)]

// required by tools.rs
#![feature(const_convert)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]

// required by panic handler
#![feature(panic_info_message)]

/*
// required by const-bitfields
#![feature(const_convert)] // optional, when using from/into conversion
#![feature(const_mut_refs)] // always required
#![feature(const_trait_impl)] // always required
*/

use core::panic::{self, PanicInfo};
// Do not remove these imports, they prevent link errors
#[allow(unused_imports)]
pub use rlibc;
pub use rlibcex;

#[panic_handler]
fn kpanic(info: &core::panic::PanicInfo<'_>) -> ! {
    log!("\nKERNEL PANIC!!!\n");
    // payload
    match info.payload().downcast_ref::<&str>() {
        Some(p) => log!("Payload: {:?}\n", p),
        None => log!("Payload: unknown\n"),
    }
    // message
    match info.message() {
        Some(msg) => log!("Message: {:?}\n", msg),
        None => log!("Message: unknown\n"),
    }
    // location
    match info.location() {
        Some(loc) => log!("Location: {}\n", loc),
        None => log!("Location: unknown"),
    }

    loop {}
}

use arrayvec::ArrayVec;

mod arch;
mod limine;
mod log;
mod memman;
mod tools;
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
    arch::init();

    // boot loader
    let (bootloader_name, bootloader_version) = limine::bootloader_info();
    log!("[ Bootloader info ]\n");
    log!("name: {}\n", core::str::from_utf8(bootloader_name).unwrap());
    log!(
        "version: {}\n",
        core::str::from_utf8(bootloader_version).unwrap()
    );

    // memory map
    log!("[ Memory Map ]\n");

    let ram_size = limine::memory_map().last().unwrap().range.1;
    unsafe { memman::map::set_global((0, ram_size)) };

    let mut map_area_pool = ArrayVec::<memman::map::MapArea, 25>::new();
    map_area_pool.push(memman::map::claim_global((0, 1000)).unwrap());
    for region in limine::memory_map() {
        let (start, end) = region.range;
        match region.typ {
            limine::MemmapEntryType::Usable => {}
            _ => {
                let ma = memman::map::claim_global(region.range)
                    .expect("Limine map entry could not be claimed!");
                if let Err(e) = map_area_pool.try_push(ma) {
                    log!("[ERROR] map_area_pool is full, limine entry will be dropped!\n");
                }
            }
        }
        let print_typ: &str = region.typ.into();
        log!("{:023} 0x{:016X} - 0x{:016X}\n", print_typ, start, end);
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
