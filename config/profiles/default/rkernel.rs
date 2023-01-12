/// First message displayed when kmain() is executed
pub const FIRST_MESSAGE: &str = "Hello World!\n";

/// Capacity of the kernel global log in bytes.
/// On overflow triggers a kernel panic
pub const STATIC_LOG_MAX_CHARACTERS: usize = 204_800;
