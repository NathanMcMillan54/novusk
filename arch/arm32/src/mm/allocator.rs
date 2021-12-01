use crate::arm32_printk;

pub unsafe fn allocator_init() {
    #[cfg(feature = "cortex_a")]
    return;

    arm32_printk!("Initializing allocator...\n");

    #[cfg(feature = "cortex_m")]
    let start = crate::cortex_m::mm::heap_start;

    #[cfg(feature = "cortex_a")]
    let start = 0;

    nmallocator::ALLOCATOR.lock().init(start as usize, 1024);
}
