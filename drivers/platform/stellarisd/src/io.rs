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

unsafe fn add_hio_driver() -> u8 {
    add_driver(&HioDriver as &'static dyn Driver);
    get_driver(SERIAL).unwrap().init();

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, add_hio_driver);

pub struct StellarisUart;

impl StellarisUart {
    pub fn new() -> Self {
        return StellarisUart;
    }

    pub fn uart_init(&mut self) {

    }
}
