pub fn device() -> &'static str {
    #[cfg(feature = "esp32")]
    return "ESP32";
}

trait Device {
    fn init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {

    }
}