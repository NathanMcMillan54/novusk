use crate::arm32_printk;
use nmallocator::LockedHeap;

#[global_allocator]
pub static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn allocator_init() {
    #[cfg(feature = "cortex_a")]
    return;

    #[cfg(feature = "cortex_m")]
    let start = crate::cortex_m::mm::heap_start;

    #[cfg(feature = "cortex_a")]
    let start = 0;

    ALLOCATOR.lock().init(start as usize, 1024);
}
