use nmallocator::WeeAlloc;

#[global_allocator]
pub static mut ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
