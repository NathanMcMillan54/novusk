use alloc::alloc::Layout;
use core::ptr::null_mut;
use crate::kernel::kernel::*;
use crate::include::asm::hlt;

#[alloc_error_handler]
pub unsafe fn alloc_error(_layout: Layout) -> ! {
    x86_printk!("\nAlloc memory error");
    x86_printk!("Memory layout:");
    x86_printk!("    {:?}", _layout);

    panic!("Alloc memory error");
}
