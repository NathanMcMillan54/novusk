use core::borrow::Borrow;
use core::fmt::{Arguments, Write};
use core::ptr::{read_volatile, write_volatile};

pub static mut ARMUART: ArmUart = ArmUart::empty();

pub struct ArmUart {
    pub addr: *mut u8,
}

impl ArmUart {
    pub const fn empty() -> Self {
        return ArmUart {
            addr: 0x0 as *mut u8,
        }
    }

    pub fn init(&mut self) {
        // Basic uart address for Rpi 2 and 3
        self.addr = 0x3F20_1000 as *mut u8;
    }

    pub fn write_byte(&self, byte: u8) {
        unsafe { write_volatile(self.addr, byte); }
    }

    pub fn read_byte(&self, buf: isize) -> u8 {
        return unsafe { read_volatile(self.addr.offset(buf)) };
    }
}

impl Write for ArmUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_byte(*b);
        }

        Ok(())
    }
}

pub fn uart_write(fmt: Arguments) {
    let mut arm_uart = unsafe { &mut ARMUART };

    if arm_uart.addr == 0x0 as *mut u8 {
        return;
    } else { arm_uart.write_fmt(fmt); }
}
