#![no_std]

use linked_list_allocator::LockedHeap;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
