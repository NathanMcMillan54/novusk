use crate::kernel::mm::mmio::Mmio;
use crate::kernel::uart::Uart;

const GPIO_BASE: usize = 0x200000;
const UART0: usize = GPIO_BASE + 0x1000;

pub struct RPiIo;

impl RPiIo {
    pub unsafe fn init(&self) -> Self {
        self.uart_init();
        return Self;
    }
}

impl Uart for RPiIo {
    unsafe fn uart_init(&self) {
        let mut mmio = Mmio::mmio(&Mmio);

        let uart_cr = UART0 + 0x30;
        mmio.mmio_write(uart_cr as *mut usize, 0x00000000);

        let gppud = GPIO_BASE + 0x94;
        mmio.mmio_write(gppud as *mut usize, 0x00000000);
    }
}
