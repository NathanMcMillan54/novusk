pub mod error;
pub mod heap;

use super::map::memory_map;

pub unsafe fn allocator_init() {
    let (bsss, bsse) = memory_map();

    heap::HEAP_ALLOCATOR.lock().init(bsss, 1024);
}
