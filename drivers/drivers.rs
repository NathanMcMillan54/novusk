use novuskinc::drivers::manager::DeviceDriverManager;
use spin::Mutex;

#[no_mangle]
pub static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
    drivers: None,
};

