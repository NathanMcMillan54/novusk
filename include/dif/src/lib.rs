#![no_std]

#[macro_use] extern crate alloc;

pub mod parse;

pub struct Dif {
    pub device_name: Option<&'static str>,
    pub peripheral_addr: Option<u32>,
    pub gpio0_addr: Option<u32>,
    pub gpio1_addr: Option<u32>,
    pub gpio2_addr: Option<u32>,
    pub gpio3_addr: Option<u32>,
    pub gpio4_addr: Option<u32>,
    pub serial_addr: Option<u32>,
    pub uart_addr: Option<u32>,
    pub fb_addr: Option<u32>,
    pub mb_addr: Option<u32>,
    pub debug: Option<bool>,
}

impl Dif {
    pub fn empty() -> Self {
        return Dif {
            device_name: None,
            peripheral_addr: None,
            gpio0_addr: None,
            gpio1_addr: None,
            gpio2_addr: None,
            gpio3_addr: None,
            gpio4_addr: None,
            serial_addr: None,
            uart_addr: None,
            fb_addr: None,
            mb_addr: None,
            debug: None
        };
    }

    pub fn set(&mut self, dif_file: &'static str) {

    }
}
