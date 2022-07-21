pub const STORAGE_STATUS_OK: u32 = 0;
pub const STORAGE_STATUS_ERR: u32 = 1;

/// A trait for storage device drivers.
pub trait Storage {
    /// Reads a storage sector's contents.
    fn read_sector(&mut self, sector: u32, buffer: &mut [u16]) -> u32 { 0 }

    /// Writes to a storage sector.
    fn write_sector(&mut self, sector: u32, buffer: &mut [u16], write: &[u8]) {  }

    /// Returns the status of the storage device, it should return ``STORAGE_STATUS_OK`` or
    /// ``STORAGE_STATUS_ERR``.
    fn status(&self) -> u32 { STORAGE_STATUS_OK }
}
