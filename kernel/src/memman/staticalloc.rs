use crate::log;
use core::alloc::GlobalAlloc;
extern crate alloc;

const MAX: usize = crate::config::STATIC_ALLOCATOR_SIZE_BYTES;

pub static GLOBAL_STATIC_ALLOCATOR: StaticAllocator<MAX> = StaticAllocator::new();

pub struct StaticAllocator<const SIZE: usize> {
    buffer: [u8; SIZE],
    bump_addr: spin::Mutex<usize>,
    refcount: spin::Mutex<usize>,
}

impl<const SIZE: usize> StaticAllocator<SIZE> {
    const fn new() -> Self {
        Self {
            // just a data holder, not actually referenced but still needed
            buffer: [0; SIZE],
            // IMPORTANT: If you are going to use this field, consult: https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html#safety
            refcount: spin::Mutex::new(0),
            // get properly inited in its `alloc` definition
            bump_addr: spin::Mutex::new(0),
        }
    }
}

unsafe impl<const SIZE: usize> GlobalAlloc for StaticAllocator<SIZE> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // expect by rust docs
        if layout.size() == 0 {
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
        *(self.refcount.lock()) += 1;

        aligned_start as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut refc = self.refcount.lock();
        let mut bumplock = self.bump_addr.lock();

        // if we are freeing the last allocated object, we can safely move bump backwards, since there are no objects in between.
        if *bumplock == (ptr as usize) + layout.size() {
            *bumplock -= layout.size();
        }

        *refc -= 1;

        // similar to an arena, we reset the whole thing when there are no objects in it.
        if *refc <= 0 {
            *bumplock = self.buffer.as_ptr() as usize;
        }
    }
}
