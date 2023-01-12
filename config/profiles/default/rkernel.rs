/// First message displayed when kmain() is executed
pub const MESSAGE_FIRST: &str = "Hello World!\n";

/// Max ammount of characters that fit into the kernel global log, sized in bytes.
///
/// If it overflows , a kernel panic is triggered
pub const LOG_STATIC_CAPACITY: usize = 204_800;

