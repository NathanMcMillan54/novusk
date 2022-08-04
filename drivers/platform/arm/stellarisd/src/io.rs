use tm4c123x_hal::gpio::gpioa::GpioControl;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::Peripherals;
use tm4c123x_hal::prelude::SysctlExt;
use tm4c123x_hal::serial::Serial;
use tm4c123x_hal::tm4c123x::{GPIO_PORTA, UART0};

pub struct StellarisUart;

impl StellarisUart {
    pub fn new() -> Self {
        return StellarisUart;
    }

    pub fn uart_init(&mut self) {

    }
}
