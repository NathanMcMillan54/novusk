#![no_std]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;

use embedded_hal::blocking::i2c::Write;
use novuskinc::fb::FrameBufferGraphics;
use ssd1306::I2CDisplayInterface;
use ssd1306::prelude::WriteOnlyDataCommand;

// pub mod i2c;
pub mod modes;
pub mod ssd1306_display;

extern "C" {
    pub(crate) fn device_ssd_handler(option: u8, pos: (u8, u8), data: u8);
}

fn ssd_display_init() {

}

fn ssd_display_end() {

}

module_init!(core_display_init, ssd_display_init);
module_end!(core_display_end, ssd_display_end);
