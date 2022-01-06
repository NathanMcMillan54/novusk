#![no_std]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This is intended for x86_64 devices that support LBA");

use storage::{StorageDev, StorageIo};

pub mod io;

pub struct Lba {
    pub start: u32,
}

impl Lba {
    pub fn new() -> Self {
        return Lba { start: 0 };
    }
}

