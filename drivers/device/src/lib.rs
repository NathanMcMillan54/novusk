#![no_std]
#![feature(const_fn_fn_ptr_basics)]

pub mod board;
pub use board::*;
pub mod macros;
pub mod traits;
pub use traits::*;

pub mod arm;
pub mod riscv;
pub mod x86;
