/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use super::ArchType;
use x86;
use x86_64;

pub mod gdt;

#[inline]
pub const fn get_arch() -> ArchType {
    ArchType::X86_64
}

pub fn init() {
    // load our GDT
    gdt::init();
}

pub mod portio {
    pub unsafe fn output_byte(port: u16, value: u8) {
        x86::io::outb(port, value)
    }

    pub unsafe fn output_word(port: u16, value: u16) {
        x86::io::outw(port, value)
    }

    pub unsafe fn output_long(port: u16, value: u32) {
        x86::io::outl(port, value)
    }

    pub unsafe fn input_byte(port: u16) -> u8 {
        x86::io::inb(port)
    }

    pub unsafe fn input_word(port: u16) -> u16 {
        x86::io::inw(port)
    }

    pub unsafe fn input_long(port: u16) -> u32 {
        x86::io::inl(port)
    }
}

pub mod cpu {
    // WARNING: Will cause a general protection fault if used outside of ring 0.
    pub unsafe fn halt() {
        x86::halt();
    }

    // WARNING: May fail with #UD if rdpid is not supported (check CPUID).
    pub unsafe fn read_id() -> u64 {
        x86::rdpid()
    }
}
