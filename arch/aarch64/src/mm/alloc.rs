use crate::aarch64_printk;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct AllocHandler;

#[global_allocator]
static ALLOCATOR: AllocHandler = AllocHandler;

unsafe impl GlobalAlloc for AllocHandler {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        loop {  }
    }
}

#[alloc_error_handler]
pub unsafe fn alloc_error(_layout: Layout) -> ! {
    aarch64_printk!("\nAlloc memory error:");
    aarch64_printk!("    Alloc Layout: {:?}", _layout);
    panic!("Memory error");
}
