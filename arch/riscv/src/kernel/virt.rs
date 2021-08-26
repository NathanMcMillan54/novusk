use super::{device, uart};

pub struct Board;

impl Board {
    pub fn new() -> Self {
        return Board;
    }
}

impl device::Device for Board {
    fn device_init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {
        unsafe { uart::Uart::new(0x1000_0000).uart_init(); }
    }

    fn name(&mut self) -> &'static str {
        return "Qemu Virt";
    }

    const UART0: usize = 0x1000_0000;
}
