use crate::aarch64_printk;
use alloc::alloc::Layout;

#[alloc_error_handler]
pub fn alloc_error(_layout: Layout) -> ! {
    aarch64_printk!("\nAlloc memory error:");
    aarch64_printk!("    Alloc Layout: {:?}", _layout);
    panic!("Memory error");
}
