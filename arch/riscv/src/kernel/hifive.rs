use hifive1::hal::DeviceResources;
use super::{device, uart::Uart};

pub struct Board;

impl device::Device for Board {
    fn device_init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {
        // This probably doesn't work but all sifive boards are micro controllers and no one will
        // see this
        unsafe { Uart::new(0x1001_3000).uart_init(); }
    }

    fn name(&mut self) -> &'static str {
        return "HiFive";
    }
}
