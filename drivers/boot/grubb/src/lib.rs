#![no_std]
#![feature(asm)]

extern "C" { pub(crate) fn grub_start() -> !;}

pub mod vga;
