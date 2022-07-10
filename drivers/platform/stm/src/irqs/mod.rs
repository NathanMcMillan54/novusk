use crate::dif::DIF_FILE;

pub mod common;

pub mod f4xx;

#[no_mangle]
pub unsafe extern "C" fn device_specific_irqs_init() {
    common::setup_interrupts();

    if DIF_FILE[0].1 == "STM32f407" {
        f4xx::stm32f4xx_specific_interrupts_setup();
    }
}
