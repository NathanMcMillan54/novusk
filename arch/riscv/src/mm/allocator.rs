use alloc::vec::Vec;
pub use nmallocator::error;
use nmallocator::WeeAlloc;

#[global_allocator]
pub static mut ALLOCATOR: WeeAlloc = WeeAlloc::INIT;

pub fn allocator_test() -> bool {
    let mut test_vec: Vec<i32> = vec![];

    for _ in 0..1024 {
        test_vec.push(1);
    }

    if test_vec.len() == 1025 {
        return true;
    } else { return false; }
}
