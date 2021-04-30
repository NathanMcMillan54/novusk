#![no_std]
#![no_main]

extern crate rlibc;

#[macro_use]
extern crate uefi;

pub mod boot;
pub mod kernel;
