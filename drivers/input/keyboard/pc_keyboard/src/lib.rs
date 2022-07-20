#![no_std]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;

use asminc::x86_64::io::{inb, outb};
use novuskinc::keyboard::KeyboardInput;

pub struct PcKeyboard {
    /// If ``irq_input`` is set to ``true`` the driver will get input from an IRQ handler.
    pub irq_input: bool,
    /// If ``irq_input`` is set to ``false`` this value needs to be set to ``Some``. It controls how
    /// often the driver checks for input, the value of ``rate`` is how many CPU cycles are between
    /// inputs.
    pub rate: Option<usize>,

    port: u32,
}

impl PcKeyboard {
    /// Creates a new PC Keyboard interface.
    pub const fn new(irqs: bool, input_rate: Option<usize>) -> Self {
        if irqs == false && input_rate.is_none() {
            panic!("Expected input_rate to be Some");
        }

        return PcKeyboard {
            irq_input: irqs,
            rate: input_rate,
            port: 0x60,
        };
    }

    /// If the keyboard is on a port that isn't ``0x60`` which it should be ``change_port`` is used
    /// to change it.
    pub fn change_port(&mut self, port: u32) {
        self.port = port;
    }

    fn irq_read_byte(&self) -> u8 {
        0
    }

    fn get_byte(&self) -> u8 {
        let byte = unsafe { inb(0x60) };

        return byte as u8;
    }
}

impl KeyboardInput for PcKeyboard {
    fn read_byte(&self) -> u8 {
        if self.irq_input {
            return self.irq_read_byte();
        } else { return self.get_byte(); }
    }

    fn read_buf(&self, buf: u8) -> &'static [u8] {
        b""
    }
}
