use core::borrow::BorrowMut;
use hifive1::clock;
use hifive1::hal::DeviceResources;
use hifive1::hal::prelude::*;
use hifive1::stdout;
use novuskinc::drivers::{Driver, DriverResult, manager::DRIVER_MANAGER};
use novuskinc::drivers::names::CONSOLE;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::prelude::*;
use crate::RISCV_DEVICE;


unsafe fn sifive_console_init() -> u8 {
    SIFIVE_CONSOLE.add_driver();
    SIFIVE_CONSOLE.init();

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, sifive_console_init);

pub struct SiFiveConsole;

impl SiFiveConsole {
    pub fn new() -> Self {
        return SiFiveConsole;
    }

    pub fn add_driver(&'static mut self) {
        unsafe { RISCV_DEVICE.console = Some(self as &'static mut dyn Driver); }
    }
}

impl KernelConsoleDriver for SiFiveConsole {
    fn write_string(&self, string: &str, x: u16, y: u16) {
        stdout::write_str(string);
    }
}

impl FrameBufferGraphics for SiFiveConsole {}

impl KeyboardInput for SiFiveConsole {}

impl Storage for SiFiveConsole {}

impl Serial for SiFiveConsole {}

impl _e310x_hal_stdout_Write for SiFiveConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for SiFiveConsole {}

impl Driver for SiFiveConsole {
    fn driver_name(&self) -> &'static str {
        "SiFive Console Driver"
    }

    fn name(&self) -> &'static str {
        CONSOLE
    }

    fn init(&self) -> DriverResult {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;
        let pins = dev_res.pins;

        let clock = clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

        stdout::configure(
            peripherals.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clock
        );

        Ok(())
    }
}

pub(crate) static mut SIFIVE_CONSOLE: SiFiveConsole = SiFiveConsole;
