use alloc::alloc::{GlobalAlloc, Layout};
use crate::kernel::kernel::arm32_printk;
use core::ptr::null_mut;
use crate::include::asm::wfe;

#[global_allocator]
pub static ALLOCATOR: AllocHandler = AllocHandler;

pub struct AllocHandler;

unsafe impl GlobalAlloc for AllocHandler {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        arm32_printk!("Alloc dealloc:\n    ptr: {:?}\n    layout: {:?}", _ptr, _layout);
        wfe();
    }
}

#[alloc_error_handler]
pub unsafe fn alloc_handler(_layout: Layout) -> ! {
    arm32_printk!("Alloc error");
    arm32_printk!("Alloc memory layout:\n    {:?}", _layout);
    wfe();
}
