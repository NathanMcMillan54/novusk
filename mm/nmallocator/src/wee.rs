use wee_alloc::WeeAlloc;

#[global_allocator]
pub static ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
