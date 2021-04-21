#![no_std]
#![feature(asm)]

extern crate kernel;
#[macro_use]
extern crate lazy_static;

#[cfg(target_arch = "arm")]
pub mod arm;

pub mod text;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
