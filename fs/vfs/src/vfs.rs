use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ops::Deref;
use core::ptr::{read, write};
use super::Dir;

pub struct Vfs {
    pub disk_size: u64,
}

impl Vfs {
    pub fn new(disk: u64) -> Self {
        return Vfs {
            disk_size: disk,
        };
    }

    pub unsafe fn write_file(address: *mut u8, write: u8) {
        *address = write;
    }

    pub unsafe fn read_file(address: *mut u8) -> u8 {
        return *address;
    }
}

pub trait VfsIo {
    fn write(name: &str, content: &[u8]) {

    }

    fn read(name: &str) -> &[u8] {
        return b"";
    }
}

