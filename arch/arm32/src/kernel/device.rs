pub(crate) const SUPPORTED_DEVICES: [&str; 3] = ["Stellaris 6965", "STM32f407", "STM32f722"];

extern "C" {
    pub(crate) fn device_init() -> (Result<(), &'static str>, &'static str);
}

pub(crate) fn device_supported(device: &str) -> bool {
    for i in 0..3 {
        if device == SUPPORTED_DEVICES[i] {
            return true;
        }
    }

    return false;
}
