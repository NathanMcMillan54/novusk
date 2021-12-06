use linked_list_allocator::{LockedHeap};
use wee_alloc::WeeAlloc;

#[global_allocator]
pub static HEAP_ALLOCATOR: WeeAlloc = WeeAlloc::INIT;

