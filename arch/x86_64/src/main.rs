#![no_std]
#![no_main]
#![feature(asm)]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;
extern crate x86_64_kernel_lib;

pub mod boot;
