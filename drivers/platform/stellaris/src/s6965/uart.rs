use core::ops::Deref;
use core::fmt::Write;
use core::ptr::write_volatile;
use cortex_m::prelude::_embedded_hal_serial_Read;
use novuskinc::drivers::{names::CONSOLE, DriverResult, Driver};
use novuskinc::prelude::*;
use tm4c123x::{Peripherals, UART0};
use tm4c123x_hal::gpio::{AF1, GpioExt};
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl::SysctlExt;
use tm4c123x_hal::time::U32Ext;

pub(crate) static mut S6965UART: S6965Uart = S6965Uart;

pub struct S6965Uart;

impl S6965Uart {
    pub fn uart0_write_byte(&self, byte: u8) {
        let dev_peripherals = unsafe { Peripherals::steal() };
        let sctl = dev_peripherals.SYSCTL.constrain();
        let clocks = sctl.clock_setup.freeze();

        let mut porta = dev_peripherals.GPIO_PORTA.split(&sctl.power_control);

        let mut uart0 = Serial::uart0(
            dev_peripherals.UART0,
            porta.pa1.into_af_push_pull::<AF1>(&mut porta.control),
            porta.pa0.into_af_push_pull::<AF1>(&mut porta.control),
            (),
            (),
            115200.bps(),
            NewlineMode::SwapLFtoCRLF,
            &clocks,
            &sctl.power_control,
        );

        let (mut tx, mut rx) = uart0.split();

        tx.write_char(byte as char);
    }

    pub fn uart0_read_byte(&self) -> u8 {
        let dev_peripherals = unsafe { Peripherals::steal() };
        let sctl = dev_peripherals.SYSCTL.constrain();
        let clocks = sctl.clock_setup.freeze();

        let mut porta = dev_peripherals.GPIO_PORTA.split(&sctl.power_control);

        let mut uart0 = Serial::uart0(
            dev_peripherals.UART0,
            porta.pa1.into_af_push_pull::<AF1>(&mut porta.control),
            porta.pa0.into_af_push_pull::<AF1>(&mut porta.control),
            (),
            (),
            115200.bps(),
            NewlineMode::SwapLFtoCRLF,
            &clocks,
            &sctl.power_control,
        );

        let (mut tx, mut rx) = uart0.split();

        rx.read().unwrap_or(0)
    }
}

impl Write for S6965Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s, 0, 0);

        Ok(())
    }
}

impl KernelConsoleDriver for S6965Uart {
    fn write_string(&self, string: &str, x: u16, y: u16) {
        for b in string.as_bytes() {
            self.uart0_write_byte(*b);
        }
    }
}

impl FrameBufferGraphics for S6965Uart {}

impl KeyboardInput for S6965Uart {}

impl Led for S6965Uart {}

impl Storage for S6965Uart {}

impl Driver for S6965Uart {
    fn driver_name(&self) -> &'static str {
        return CONSOLE;
    }

    fn name(&self) -> &'static str {
        return "S6965 Uart Driver";
    }

    fn init(&self) -> Option<DriverResult> {
        Some(Ok(()))
    }
}

pub fn uart_init() {

}
