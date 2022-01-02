use alloc::vec::Vec;
pub use nmallocator::error;
use nmallocator::WeeAlloc;

#[global_allocator]
pub static mut ALLOCATOR: WeeAlloc = WeeAlloc::INIT;

pub fn allocator_test() -> bool {

    return true;
}
