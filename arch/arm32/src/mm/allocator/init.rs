use super::ALLOCATOR;
use cortex_m_rt::heap_start;

pub(crate) unsafe fn allocator_init() {
    ALLOCATOR.init(heap_start() as usize, 1024);
}