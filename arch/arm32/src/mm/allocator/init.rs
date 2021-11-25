use super::{test_allocator, ALLOCATOR};

#[cfg(feature = "cortex_m")]
use crate::cortex_m::heap_start;

#[cfg(feature = "cortex_a")]
use crate::cortex_a::heap_start;

pub(crate) unsafe fn allocator_init() {
    ALLOCATOR.lock().init(heap_start() as usize, 1024);
    // test_allocator();
}
