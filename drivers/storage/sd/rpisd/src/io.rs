use crate::RpiSdCard;
use storage::StorageIo;

impl StorageIo for RpiSdCard {
    fn read_sector(&mut self, sectors: u32, buffer: &mut [u16]) -> u32 {
        0
    }

    fn write_sector(&mut self, sector: u32, buffer: &mut [u16], write: &[u8]) {

    }
}
