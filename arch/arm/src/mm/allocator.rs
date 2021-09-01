use crate::include::asm::wfe;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct Allocator;

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        wfe();
    }
}
