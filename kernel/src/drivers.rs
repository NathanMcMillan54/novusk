use alloc::vec;
use novuskinc::drivers::manager::*;

#[no_mangle]
pub static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
    drivers: vec![]
};

