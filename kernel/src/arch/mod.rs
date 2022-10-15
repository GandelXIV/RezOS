mod arch_x86;

pub enum ArchType {
    X86_64,
}

#[cfg(target_arch = "x86_64")]
pub use arch_x86::*;
