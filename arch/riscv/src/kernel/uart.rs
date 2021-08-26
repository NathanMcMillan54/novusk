use core::convert::TryInto;
use core::fmt::{Arguments, Write, Result};

static mut UART_ADDRESS: usize = 0x0;

pub struct Uart {
    pub address: usize,
}

impl Uart {
    pub fn new(uart_address: usize) -> Self {
        return Self { address: uart_address };
    }

    pub unsafe fn uart_init(&mut self) {
        let mut uart_addr = self.address as *mut u8;

        let lcr= (1 << 0) | (1 << 1);
        let divisor = 592 as u16;
        let divisor_least = (divisor & 0xff).try_into().unwrap();
        let divisor_most = (divisor >> 8).try_into().unwrap();

        // Write the variable above to thr UART address
        // In the future this might be done with mmio because this is literally what that does
        uart_addr.add(3).write_volatile(lcr);
        uart_addr.add(2).write_volatile(1 << 0);
        uart_addr.add(1).write_volatile(1 << 0);
        uart_addr.add(3).write_volatile(lcr | 1 << 7);
        uart_addr.add(0).write_volatile(divisor_least);
        uart_addr.add(1).write_volatile(divisor_most);
        uart_addr.add(3).write_volatile(lcr);

        self.finish_init();
    }

    unsafe fn finish_init(&mut self) {
        UART_ADDRESS = self.address;
    }

    pub unsafe fn current_address() -> usize {
        return UART_ADDRESS;
    }

    // I/O
    pub fn write_byte(&mut self, byte: u8) {
        let mut uart = self.address as *mut u8;
        unsafe { uart.add(0).write_volatile(byte); }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    pub fn read(&mut self) -> u8 {
        let mut uart = self.address as *mut u8;
        return unsafe { uart.add(0).read_volatile() };
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe { self.write_string(s); }
        Ok(())
    }
}
