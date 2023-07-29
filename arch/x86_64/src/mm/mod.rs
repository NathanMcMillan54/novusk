use wee_alloc::WeeAlloc;

pub mod memory;

#[global_allocator]
static mut ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
