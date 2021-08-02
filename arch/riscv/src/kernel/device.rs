pub trait Device {
    fn device_init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {

    }

    fn name(&mut self) -> &'static str {
        return "None";
    }
}
