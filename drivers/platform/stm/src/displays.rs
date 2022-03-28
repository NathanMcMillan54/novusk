use core::fmt::Write;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use ssd1306::prelude::{DisplayConfig, DisplaySize128x64, I2CInterface};
use ssd1306::prelude::DisplayRotation::Rotate0;
use stm32f4xx_hal::gpio::{Alternate, OpenDrain, Pin};
use stm32f4xx_hal::i2c::I2c;
use crate::stm32f4xx::get_clocks;
use stm32f4xx_hal::pac::{I2C1, Peripherals};
use stm32f4xx_hal::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn device_ssd_handler(option: u8, pos: (u8, u8), data: u8) {
    let peripherals = Peripherals::steal();

    let gpiob = peripherals.GPIOB.split();

    let scl = gpiob.pb8.into_alternate().internal_pull_up(true).set_open_drain();
    let sda = gpiob.pb9.into_alternate().internal_pull_up(true).set_open_drain();
    let i2c = peripherals.I2C1.i2c((scl, sda), 400.kHz(), &get_clocks());

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, Rotate0);

    if option == 0 {
        display.init().unwrap();
    } else if option == 1 {
        display.set_row(pos.0);
        display.set_column(pos.1);
        display.into_terminal_mode().write_char(data as char);
    } else if option == 2 {
        display.set_row(pos.0);
        display.set_row(pos.1);
        display.into_buffered_graphics_mode().set_pixel(pos.0 as u32, pos.1 as u32, true);
    }
}
