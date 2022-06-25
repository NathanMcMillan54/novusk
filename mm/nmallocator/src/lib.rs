#![no_std]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;

use wee_alloc::WeeAlloc;

mod error;

/// This module contains functions that ``kernel/tests/`` use.
pub mod tests;

#[global_allocator]
pub static mut GLOBAL_ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
