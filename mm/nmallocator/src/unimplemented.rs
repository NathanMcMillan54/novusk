use core::alloc::{GlobalAlloc, Layout};
use core::ops::Add;

pub struct NMAllocator {
    allocations: u32,
}

unsafe impl GlobalAlloc for NMAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocations.add(1);

        return 0x0 as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        return;
    }
}

#[global_allocator]
pub static ALLOCATOR: NMAllocator = NMAllocator {
    allocations: 0,
};
