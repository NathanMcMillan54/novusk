pub mod allocator;

pub(crate) unsafe fn arm_mm_init() {
    allocator::allocator_init();
}
