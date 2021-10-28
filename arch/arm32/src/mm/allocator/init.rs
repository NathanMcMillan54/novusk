use super::{test_allocator, ALLOCATOR};
use cortex_m_rt::heap_start;

pub(crate) unsafe fn allocator_init() {
    ALLOCATOR.init(heap_start() as usize, 1024);
    test_allocator();
}
