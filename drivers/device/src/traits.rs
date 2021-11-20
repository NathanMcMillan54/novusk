pub trait Device {
    fn name(&self) -> &'static str {
        return "";
    }

    fn serial_io_init(&self) {

    }

    fn time_init(&self) {

    }

    fn disable_wdt(&self) {

    }

    fn enable_wdt(&self) {

    }

    fn gpio_init(&self) {

    }

    #[cfg(not(target_arch = "x86_64"))]
    fn write_bytes(&self, bytes: &[u8]) {

    }

    fn test(&self) {

    }
}

pub trait DeviceDriver {
    fn init(&self) {

    }

    fn name(&self) -> &'static str {
        return "";
    }
}
