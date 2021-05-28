use core::{fmt, ptr};

static mut UART0: *mut u8 = 0 as *mut u8;

pub struct Uart0;

impl Uart0 {
    pub unsafe fn uart0_init(&mut self, uart: *mut u8) {
        UART0 = uart;
    }

    pub fn write_byte(&mut self, b: u8) {
        unsafe { ptr::write_volatile(UART0, b); }
    }

    pub fn write_bytes(&mut self, arg: &[u8]) {
        for byte in arg {
            self.write_byte(*byte);
        }
    }

    pub fn write_string(&mut self, arg: &str) {
        let byte = arg.as_bytes();
        self.write_bytes(byte);
    }
}

impl fmt::Write for Uart0 {
    fn write_str(&mut self, args: &str) -> fmt::Result {
        self.write_string(args);
        Ok(())
    }
}
