// For initializing the device

extern "C" {
    pub fn device_init() -> (Result<(), &'static str>, &'static str);
}

pub fn initialize_device() -> (Result<(), &'static str>, &'static str) {
    return unsafe { device_init() };
}
