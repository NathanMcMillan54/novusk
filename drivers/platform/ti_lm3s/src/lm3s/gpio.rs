use tm4c123x_hal::gpio::{GpioExt};
use tm4c123x_hal::gpio::gpioa::Parts;
use tm4c123x_hal::prelude::SysctlExt;
use crate::Peripherals;

pub unsafe fn setup_gpio() {
    let peripherals = Peripherals::steal();
    let mut gpio = peripherals.GPIO_PORTA.split(&peripherals.SYSCTL.constrain().power_control);
}