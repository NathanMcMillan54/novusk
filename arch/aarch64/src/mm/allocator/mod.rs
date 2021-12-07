pub mod wee;

use crate::aarch64_printk;
use super::map::memory_map;

// This is mostly a test since the allocator is initialized at compile time
pub fn allocator_init() {
    wee::ALLOCATOR.lock();

    let mut a_vec = vec![0];

    for i in 0..1000 {
        a_vec.push(i);
    }
}
