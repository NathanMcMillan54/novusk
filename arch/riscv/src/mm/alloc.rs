use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use crate::riscv_printk;

pub struct Allocator;

#[global_allocator]
pub static GLOBAL_ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        loop {  }
    }
}

#[alloc_error_handler]
unsafe fn alloc_error(_layout: Layout) -> ! {
    riscv_printk!("Alloc memory layout:");
    riscv_printk!("    {:?}", _layout);
    panic!("Alloc memory error");
}
