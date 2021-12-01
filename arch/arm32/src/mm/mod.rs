pub mod allocator;

pub unsafe fn memory_init() {
    allocator::allocator_init();
}
