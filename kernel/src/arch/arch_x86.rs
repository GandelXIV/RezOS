use super::ArchType;
use x86;

pub const fn get_arch() -> ArchType {
    ArchType::X86_64
}

pub mod portio {
    pub unsafe fn output_byte(port: u16, value: u8) {
        x86::io::outb(port, value)
    }
}