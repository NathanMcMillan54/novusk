use nmallocator::WeeAlloc;

#[global_allocator]
pub static ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
