use alloc_cortex_m::CortexMHeap;
use crate::include::asm::wfe;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use cortex_m_rt::heap_start;


#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub(crate) unsafe fn allocator_init() {
    ALLOCATOR.init(heap_start() as usize, 1024);

    // Test if it worked
    vec![1];
}
