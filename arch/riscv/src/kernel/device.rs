use crate::kernel::board::Board;

pub const DEVICE: Board = Board;

pub trait Device {
    fn device_init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {

    }

    fn blink(&mut self) {

    }

    fn name(&mut self) -> &'static str {
        return "None";
    }

    const UART0: usize = 0;
}
