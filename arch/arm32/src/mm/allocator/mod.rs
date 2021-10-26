use alloc_cortex_m::CortexMHeap;

pub mod error;
pub mod init;

#[global_allocator]
pub(crate) static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
