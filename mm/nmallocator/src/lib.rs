#![no_std]
#![feature(alloc_error_handler)]

#[macro_use] extern crate cfg_if;

pub mod error;

#[cfg(target_arch = "riscv32")]
pub mod riscv;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(feature = "linked_list")]
pub use linked_list_allocator::LockedHeap;

#[cfg(feature = "wee_allocator")]
pub use wee_alloc::WeeAlloc;
