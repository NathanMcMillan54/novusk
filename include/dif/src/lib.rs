#![no_std]

#[macro_use] extern crate alloc;

use alloc::vec::Vec;
use core::borrow::Borrow;
use rjson::parse;

#[path = "../../../lib/libcopy.rs"]
pub mod libcopy;

pub(crate) mod json;
pub mod parse;

use json::*;

pub struct Dif {
    pub device_name: &'static str,
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
    pub const fn empty() -> Self {
        return Dif {
            device_name: "",
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

    pub fn new(name: &'static str,
               periph_addr: Option<u32>,
               gpio0: Option<u32>,
               gpio1: Option<u32>,
               gpio2: Option<u32>,
               gpio3: Option<u32>,
               gpio4: Option<u32>,
               serial: Option<u32>,
               uart: Option<u32>,
               fb: Option<u32>,
               mb: Option<u32>,
               debug: Option<bool>) -> Self {

        return Dif {
            device_name: name,
            peripheral_addr: periph_addr,
            gpio0_addr: gpio0,
            gpio1_addr: gpio1,
            gpio2_addr: gpio2,
            gpio3_addr: gpio3,
            gpio4_addr: gpio4,
            serial_addr: serial,
            uart_addr: uart,
            fb_addr: fb,
            mb_addr: mb,
            debug: debug,
        }
    }

    pub fn set(&mut self, dif_file: &'static str) {
        let json_data: Vec<char> = dif_file.chars().collect();
        let mut index = 0;

        let read_data = parse::<JsonValue, JsonArray, JsonObject, JsonValue>(&*json_data, &mut index);

        if read_data.is_none() {
            panic!("DIF is either empty or not formatted properly");
        }
    }
}
