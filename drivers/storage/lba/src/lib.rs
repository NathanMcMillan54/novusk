#![no_std]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This is intended for x86_64 devices that support LBA");

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;

pub mod io;

pub struct Lba {
    pub disk_start: u32,
}

impl Lba {
    pub fn new() -> Self {
        return Lba { disk_start: 0 };
    }
}
