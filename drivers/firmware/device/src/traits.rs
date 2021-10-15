pub trait Device {
    fn name(&self) -> &'static str {
        return "";
    }

    fn serial_io_init(&self) {

    }

    fn time_init(&self) {

    }

    fn gpio_init(&self) {

    }
}

pub trait DeviceDriver {
    fn init(&self) {

    }

    fn name(&self) -> &'static str {
        return "";
    }
}
