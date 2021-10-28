use alloc_cortex_m::CortexMHeap;

pub mod error;
pub mod init;
pub(crate) use init::allocator_init;

#[global_allocator]
pub(crate) static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
