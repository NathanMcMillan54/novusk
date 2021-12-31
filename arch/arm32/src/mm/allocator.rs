use crate::arm32_printk;

#[cfg(feature = "cortex_m")]
#[global_allocator]
pub static mut ALLOCATOR: nmallocator::LockedHeap = nmallocator::LockedHeap::empty();

#[cfg(feature = "cortex_a")]
#[global_allocator]
pub static mut ALLOCATOR: nmallocator::WeeAlloc = nmallocator::WeeAlloc::INIT;

pub unsafe fn allocator_init() {
    #[cfg(feature = "cortex_a")]
    vec![0, 1, 2].push(3);

    #[cfg(feature = "cortex_m")]
    ALLOCATOR.lock().init(cortex_m_rt::heap_start() as usize, 1024);
}
