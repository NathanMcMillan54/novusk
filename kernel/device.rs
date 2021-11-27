// For initializing the device and storing some information

pub static mut DEVICE_NAME: &'static str = "Unknown";

extern "C" {
    pub fn device_init() -> (Result<(), &'static str>, &'static str);
}

pub fn initialize_device() -> (Result<(), &'static str>, &'static str) {
    return unsafe { device_init() };
}
