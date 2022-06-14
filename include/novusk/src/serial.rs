use core::fmt::{Arguments, Result, Write};
use core::ptr::{read_volatile, write_volatile};

extern "C" {
    /// ``early_serial_init`` is used to initialize a ``SimpleUart`` driver, this is usually used
    /// for debugging and comes before the main ``Uart`` driver is initialized.
    pub fn early_serial_init();
}

/// ``SimpleUart`` is a serial driver mainly used for early printing and debugging
#[derive(Debug, Copy, Clone)]
pub struct SimpleUart {
    /// The name field is for the SimpleUart's
    pub name: &'static str,
    /// This address is used for printing
    pub output_addr: *mut u8,
    /// This address is used for receiving input
    pub input_addr: *mut u8,
}

impl SimpleUart {
    /// Creates a new ``SimpleUart``
    pub fn new() -> Self {
        return SimpleUart {
            name: "Unknown",
            output_addr: 0x0 as *mut u8,
            input_addr: 0x0 as *mut u8,
        };
    }

    /// Creates a new ``SimpleUart``
    pub const fn empty() -> Self {
        return SimpleUart {
            name: "Unknown",
            output_addr: 0x0 as *mut u8,
            input_addr: 0x0 as *mut u8,
        };
    }

    /// Set ``SimpleUart`` addresses 
    pub fn set_addrs(&mut self, out_addr: *mut u8, in_addr: *mut u8) {
        self.output_addr = out_addr;
        self.input_addr = in_addr;
    }

    /// Writes a byte to the ``output_addr`` field's value
    pub unsafe fn serial_write_byte(&self, byte: u8) {
        write_volatile(self.output_addr, byte);
    }
    
    /// Reads a byte from the ``input_addr`` field's value
    pub unsafe fn serial_read_byte(&self) -> u8 {
        return read_volatile(self.input_addr);
    }
}

impl Write for SimpleUart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.as_bytes() {
            unsafe { self.serial_write_byte(*byte); }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! gen_simpleuart {
    () => {
        #[no_mangle]
        pub static mut KERNEL_SIMPLEUART: $crate::serial::SimpleUart = $crate::serial::SimpleUart::empty();
    }
}
