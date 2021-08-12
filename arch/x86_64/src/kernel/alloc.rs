use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use super::kernel::*;
use crate::include::asm::hlt;

#[global_allocator]
static ALLOCATOR: AllocHandler = AllocHandler;

pub struct AllocHandler;

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
    x86_printk!("Alloc memory error:");
    x86_printk!("   Memory layout:\n        {:?}", _layout);
    hlt();
}
