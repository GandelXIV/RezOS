use crate::log;
use core::alloc::GlobalAlloc;
extern crate alloc;

const MAX: usize = crate::config::STATIC_ALLOCATOR_SIZE_BYTES;

#[global_allocator]
static GLOBAL_STATIC_ALLOCATOR: StaticAllocator<MAX> = StaticAllocator::new();

struct StaticAllocator<const SIZE: usize> {
    buffer: [u8; SIZE],
    bump_addr: spin::Mutex<usize>,
}

impl<const SIZE: usize> StaticAllocator<SIZE> {
    const fn new() -> Self {
        Self {
            // just a data holder, not actually referenced but still needed
            buffer: [0; SIZE],
            // get properly inited in its `alloc` definition
            bump_addr: spin::Mutex::new(0),
        }
    }
}

unsafe impl<const SIZE: usize> GlobalAlloc for StaticAllocator<SIZE> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // expect by rust docs
        if layout.size() == 0 {
            log!("ZERO!");
            return core::ptr::null_mut();
        }

        let mut bumplock = self.bump_addr.lock();

        // kinda bad design, but its late and works so idc
        if *bumplock == 0 {
            *bumplock = self.buffer.as_ptr() as usize;
        }

        // we take the first free chunk, or a bit more for it to be aligned
        let first_start = *bumplock;
        let aligned_start = if first_start % layout.align() > 0 {
            first_start + layout.align() - (first_start % layout.align())
        } else {
            first_start
        };
        // if the returned chunk is greater than last address of buffer, we ran out of memory :(
        if aligned_start + layout.size() >= self.buffer.as_ptr() as usize + self.buffer.len() {
            log!("NO SPACE :(!");
            return core::ptr::null_mut();
        }
        // upadte that ting
        *bumplock = aligned_start + layout.size();

        aligned_start as *mut u8
    }
    // we dont free this lmao
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {}
}
