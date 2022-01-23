pub use nmallocator::error;

#[global_allocator]
pub static mut ALLOCATOR: nmallocator::WeeAlloc = nmallocator::WeeAlloc::INIT;
