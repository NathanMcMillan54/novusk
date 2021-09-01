pub mod allocator;
pub mod error;
pub use allocator::ALLOCATOR;

use allocator::allocator_init;

pub unsafe fn early_memory_init() {
    allocator_init();
}
