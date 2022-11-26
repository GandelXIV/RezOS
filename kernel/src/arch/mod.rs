mod amd64;

pub enum ArchType {
    X86_64,
    Arm64,
}

#[cfg(target_arch = "x86_64")]
pub use amd64::*;
