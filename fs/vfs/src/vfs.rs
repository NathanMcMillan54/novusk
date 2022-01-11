use alloc::vec::Vec;
use core::borrow::Borrow;
use core::ptr::{read, read_volatile, write, write_volatile};
use super::Dir;

pub trait VfsIo {
    fn write(&mut self, name: &str, content: &[u8]) {

    }

    fn read(&mut self, name: &str) -> &[u8] {
        return b"";
    }
}

pub struct Vfs {
    pub disk_size: u64,
}

impl Vfs {
    pub fn new(disk: u64) -> Self {
        return Vfs {
            disk_size: disk,
        };
    }

    pub unsafe fn write_file(&self, address: *mut u8, write: u8) {
        write_volatile(address, write);
    }

    pub unsafe fn read_file(&self, address: *mut u8) -> u8 {
        return read_volatile(address);
    }
}

pub struct VfsFile {
    pub address: *mut u8,
    pub len: u32,
}

impl VfsFile {
    pub fn new(address: *mut u8) -> Self {
        return VfsFile {
            address: address,
            len: 0,
        }
    }
}

impl VfsIo for VfsFile {
    fn write(&mut self, name: &str, content: &[u8]) {
        for o in 0..content.len() {
            for b in content {
                unsafe { write(self.address.offset(self.len as isize + o as isize), *b); }
            }
        }

        self.len += content.len() as u32;
    }

    fn read(&mut self, name: &str) -> &[u8] {
        let mut ret = &[];

        for o in 0..self.len {
            unsafe {
                ret.iter().map(|x| read(self.address.offset(o as isize)));
            }
        }

        ret.map(|x| b'A');

        return ret;
    }
}
