#![no_std]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;

use wee_alloc::WeeAlloc;

mod error;

#[cfg(test)]
pub mod tests;

#[global_allocator]
pub static mut GLOBAL_ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
