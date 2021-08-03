#![no_std]
#![crate_type = "staticlib"]

pub mod memory;
pub mod types;

pub use types::*;
pub use core::ffi::c_void;
