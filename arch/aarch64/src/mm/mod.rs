pub mod allocator;
pub mod map;

pub unsafe fn memory_init() {
    allocator::allocator_init();
}
