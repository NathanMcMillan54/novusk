use crate::xpirntk;

pub fn device() -> &'static str {
    #[cfg(feature = "esp32")]
    return "ESP32";
}

pub trait Device {
    fn init(&mut self) {
        self.get_peripherals();
        self.time_init();
        self.io_init();

        xpirntk!("{} board initialized", device());
    }

    fn get_peripherals(&mut self) {

    }

    fn time_init(&mut self) {

    }

    fn io_init(&mut self) {

    }
}