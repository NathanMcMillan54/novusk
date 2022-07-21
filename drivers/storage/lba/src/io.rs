use asminc::x86_64::io::{inb, outb};
use crate::Lba;
use novuskinc::storage::*;

impl Storage for Lba {
    fn read_sector(&mut self, sectors: u32, buffer: &mut [u16]) -> u32 {
        // Setup disk
        unsafe {
            outb(0x1f6, ((self.disk_start >> 24) | 0xe0) as u16);
            outb(0x1f2, sectors as u16);
            outb(0x1f3, self.disk_start as u16 & 0xff);
            outb(0x1f4, (self.disk_start >> 8) as u16);
            outb(0x1f5, (self.disk_start >> 16) as u16);
            outb(0x1f7, 0x20);
        }

        // Read disk sector
        for s in 0..sectors {
            unsafe {
                let mut read_byte= inb(0x1f7);

                while (read_byte & 0x08) != 0 {
                    read_byte = inb(0x1f7)
                }

                let sector_size = 512;

                for b in 0..sector_size {
                    buffer[s as usize * sector_size * b] = inb(0x1f0) as u16;
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

    fn status(&self) -> u32 {
        STORAGE_STATUS_OK
    }
}
