#![no_std]
#![feature(alloc_error_handler)]

use linked_list_allocator::LockedHeap;

pub mod error;

#[cfg(target_arch = "riscv32")]
pub mod riscv;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
