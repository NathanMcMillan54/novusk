#![no_std]

pub mod konsole;

pub trait Console {
    fn write_byte(&mut self, byte: u8) {

    }

    fn write_string(&mut self, string: &str) {

    }
}
