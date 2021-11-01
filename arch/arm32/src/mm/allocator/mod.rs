use alloc::vec::Vec;
use linked_list_allocator::LockedHeap;

pub mod error;
pub mod init;
pub(crate) use self::init::allocator_init;
pub(self) use nmallocator::ALLOCATOR;

pub(crate) fn test_allocator() {
    let mut test_vec = vec![];

    for i in 0..100 {
        test_vec.push(i);
    }
}
