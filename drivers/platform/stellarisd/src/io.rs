use novuskinc::dif::get_dif_value;
use novuskinc::drivers::Driver;
use novuskinc::drivers::{add_driver, get_driver};
use novuskinc::drivers::names::SERIAL;
use novuskinc::kernel::types::KernelFunctionName;
use tm4c123x_hal::gpio::gpioa::GpioControl;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::Peripherals;
use tm4c123x_hal::prelude::SysctlExt;
use tm4c123x_hal::serial::Serial;
use tm4c123x_hal::tm4c123x::{GPIO_PORTA, UART0};
use hio::HioDriver;

unsafe fn init_printing_driver() -> u8 {
    if get_driver(get_dif_value("PrintingMethod")).is_none() {
        return 1;
    }

    if get_driver(get_dif_value("PrintingMethod")).unwrap().init().is_ok() {
        0
    } else { 1 }
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, init_printing_driver);

pub struct StellarisUart;

impl StellarisUart {
    pub fn new() -> Self {
        return StellarisUart;
    }

    pub fn uart_init(&mut self) {

    }
}
