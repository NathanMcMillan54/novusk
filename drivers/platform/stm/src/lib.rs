#![no_std]

#[macro_use] extern crate asminc;
#[macro_use] extern crate novuskinc;

#[path = "dif.rs"]
mod dif;

mod displays;
pub mod irqs;
pub mod stm32f4xx;

fn stm_init() {
    if dif::DIF_FILE[0] == "STM32F4xx" {
        stm32f4xx::setup_stm32f407();
    } else { panic!("Wrong DIF file should be using a STM32Xxxx DIF not a {} DIF", dif::DIF_FILE[0]); }
}

fn stm_end() {

}

module_init!(early_device_init, stm_init);
module_end!(early_device_end, stm_end);
