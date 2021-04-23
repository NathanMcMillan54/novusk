use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{null_mut};

struct AllocManagement;

unsafe impl GlobalAlloc for AllocManagement {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        loop {  }
    }
}

#[global_allocator]
static ALLOCATOR: AllocManagement = AllocManagement;
