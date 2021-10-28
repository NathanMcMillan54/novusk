use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;

pub mod error;
pub mod init;
pub(crate) use init::allocator_init;

#[cfg(feature = "cortex_m")]
#[global_allocator]
pub(crate) static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub(crate) fn test_allocator() {
    let mut test_vec = vec![];

    for i in 0..100 {
        test_vec.push(i);
    }
}
