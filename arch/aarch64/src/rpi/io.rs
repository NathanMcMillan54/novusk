use crate::kernel::mm::mmio::Mmio;
use crate::kernel::uart::Uart;

pub struct RPiIo;

impl RPiIo {
    pub fn init(&self) -> Self {
        return Self;
    }
}

impl Uart for RPiIo {
    unsafe fn uart_init() {
        let mut mmio = Mmio::mmio(&Mmio);
    }
}
