pub use nmallocator::error;

#[cfg(not(feature = "no_alloc"))]
#[global_allocator]
pub static mut ALLOCATOR: nmallocator::WeeAlloc = nmallocator::WeeAlloc::INIT;
