use alloc::vec;
use novuskinc::drivers::Driver;
use novuskinc::drivers::manager::*;

#[no_mangle]
pub static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
    drivers: vec![]
};

#[no_mangle]
pub unsafe extern "C" fn add_driver(driver: &'static mut dyn Driver) {
    DEVICE_DRIVERS.add_driver(driver);
}
