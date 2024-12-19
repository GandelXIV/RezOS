/// First message displayed when kmain() is executed
pub const MESSAGE_FIRST: &str = "Hello World!\n";

/// Max ammount of characters that fit into the kernel global log, sized in bytes.
///
/// If it overflows , a kernel panic is triggered
pub const LOG_STATIC_CAPACITY: usize = 204_800;

/// The static allocator allocator is used by rust prior to any other being setup. It takes space in the kernel binary itself. Using this we can size its size.
///
pub const STATIC_ALLOCATOR_SIZE_BYTES: usize = 100_000;
