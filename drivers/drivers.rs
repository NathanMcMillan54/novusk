use novuskinc::drivers::manager::DeviceDriverManager;

#[no_mangle]
pub static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
    drivers: None,
};
