#![no_std]

use linked_list_allocator::LockedHeap;

pub mod x86_64;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
