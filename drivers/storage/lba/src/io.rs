use crate::Lba;
use storage::{StorageDev, StorageIo};
use x86_64::instructions::port::{Port};

impl StorageIo for Lba {
    fn read_sector(&mut self, sectors: u32, buffer: &mut [u16]) -> u32 {
        // Setup disk
        unsafe {
            Port::new(0x1f6).write((self.start >> 24) | 0xe0);
            Port::new(0x1f2).write(sectors);
            Port::new(0x1f3).write(self.start & 0xff);
            Port::new(0x1f4).write(self.start >> 8);
            Port::new(0x1f5).write(self.start >> 16);
            Port::new(0x1f7).write(0x20 as u8);
        }

        // Read disk
        for s in 0..sectors {
            unsafe {
                let mut read_byte: u8 = Port::new(0x1f7).read();

                while (read_byte & 0x08) != 0 {
                    read_byte = Port::new(0x1f7).read();
                }

                let sector_size = 512;

                for b in 0..sector_size {
                    buffer[s as usize * sector_size * b] = Port::new(0x1f0).read();
                }
            }
        }

        0
    }

    fn write_sector(&mut self, sector: u32, buffer: &mut [u16], write: &[u8]) {
        // Setup disk
        unsafe {
            Port::new(0xf17).write(0x20 as u8);
        }
    }
}
