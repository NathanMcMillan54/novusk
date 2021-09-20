use alloc::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr::null_mut;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Alloc memory error {:?}", layout);
}

pub struct Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;


unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        loop {  }
    }
}
