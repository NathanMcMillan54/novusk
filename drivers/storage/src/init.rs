use crate::empty::EmptyStorage;

pub unsafe fn storage_init() {
    start_module!(storage_device_init, storage_device_end);
}
