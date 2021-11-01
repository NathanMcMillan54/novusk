use super::{TempFs, Vfs};

impl Vfs for TempFs {
    fn write(&self, byte: u8, addr: usize) {
        
    }

    fn read(&self, buf: u8, addr: usize) -> u8 {
        0
    }
}
