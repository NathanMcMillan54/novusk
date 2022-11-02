pub mod allocator;
pub(crate) mod linker;

pub unsafe fn memory_init() {
    allocator::allocator_init();
}
