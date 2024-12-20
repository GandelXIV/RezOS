use core::alloc::GlobalAlloc;
extern crate alloc;

type MainAllocator = RootAllocator;

#[global_allocator]
static GLOBAL_ALLOC: MainAllocator = MainAllocator::new();

struct RootAllocator {}

impl RootAllocator {
    const fn new() -> Self {
        Self {}
    }
}

unsafe impl GlobalAlloc for RootAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        super::staticalloc::GLOBAL_STATIC_ALLOCATOR.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        super::staticalloc::GLOBAL_STATIC_ALLOCATOR.dealloc(ptr, layout);
    }
}
