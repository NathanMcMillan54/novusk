use crate::{StorageDev, StorageIo};

static mut STORAGE_DEVICE: StorageDev<EmptyStorage> = StorageDev {
    name: "empty",
    start: 0,
    stroage_io: EmptyStorage { init_success: false },
};

pub struct EmptyStorage {
    pub init_success: bool,
}

impl EmptyStorage {
    pub fn new() -> Self {
        return EmptyStorage { init_success: false };
    }
}

impl StorageIo for EmptyStorage {
    fn read_sector(&mut self, sectors: u32, buffer: &mut [u16]) -> u32 {
        0
    }

    fn write_sector(&mut self, sector: u32, buffer: &mut [u16], write: &[u8]) {

    }
}

unsafe fn emptystorage_init() {
    let reading_sector = 1;
    let buffer = &mut [0; 512];

    if STORAGE_DEVICE.stroage_io.read_sector(reading_sector, buffer) == 0 {
        STORAGE_DEVICE.stroage_io.init_success = true;
    }
}

unsafe fn emptystorage_end() {
    if !STORAGE_DEVICE.stroage_io.init_success {
        panic!("Initialized EmptyStorage unsuccessfully");
    }
}

#[cfg(feature = "empty")]
module_init!(storage_device_init, emptystorage_init);

#[cfg(feature = "empty")]
module_end!(storage_device_end, emptystorage_end);
