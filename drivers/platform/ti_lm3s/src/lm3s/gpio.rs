use core::borrow::Borrow;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::prelude::SysctlExt;
use crate::Peripherals;

pub unsafe fn setup_gpio() {
    /*let gpiob = peripherals.GPIO_PORTB.split(&LM3S6965_DEV.systcl.as_ref().unwrap().power_control);
    LM3S6965_DEV.gpioa = Some(gpioa);
    LM3S6965_DEV.gpiob = Some(gpiob);*/
}
