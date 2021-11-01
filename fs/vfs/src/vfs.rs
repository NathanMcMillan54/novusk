use core::ptr::{read, write};
use super::Dir;

pub trait Vfs {
    fn write(&self, byte: u8, addr: usize) {
        //ROOT_DIR.new_dir("temp/");
        unsafe { write(byte as *mut usize, addr); }
    }

    fn read(&self, buf: u8, addr: usize) -> u8 {
        unsafe { return read(addr as *const u8); }
    }
}

