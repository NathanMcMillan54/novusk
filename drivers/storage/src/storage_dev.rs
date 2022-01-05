use alloc::boxed::Box;
use core::ops::Deref;

pub trait StorageIo {
    fn read_sector(&mut self, sectors: u32, buffer: &mut [u16]) -> u32 { 0 }

    fn write_sector(&mut self, sector: u32, buffer: &mut [u16], write: &[u8]) {  }
}


pub struct StorageDev<T: StorageIo> {
    pub name: &'static str,
    pub start: u32,
    pub stroage_io: T,
}

impl<T> StorageDev<T>
where
    T: StorageIo,
{
    pub fn new(device_name: &'static str, device_start: u32, io: T) -> Self {
        return StorageDev {
            name: device_name,
            start: device_start,
            stroage_io: io,
        };
    }
}

