use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult, names};
use novuskinc::drivers::manager::DEVICE_DRIVERS;
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::prelude::*;
use novuskinc::serial::{SimpleUart, Serial};
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::Peripherals;
use tm4c123x_hal::prelude::SysctlExt;
use tm4c123x_hal::serial::UART0;
use tm4c123x_hal::sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, SystemClock};

const UART0_ADDR: u32 = 0x4000_C000;

/// A UART driver for the LM3S6965 using its builtin UART0
pub struct LM3SUart {
    uart0: UART0,
}

impl LM3SUart {
    pub fn new() -> Self {
        let peripherals = unsafe { Peripherals::steal() };

        return LM3SUart {
            uart0: peripherals.UART0,
        };
    }

    pub fn uart_init(&mut self) {

    }
}

impl KernelConsoleDriver for LM3SUart {}

impl FrameBufferGraphics for LM3SUart {}

impl KeyboardInput for LM3SUart {}

impl Storage for LM3SUart {}

impl Serial for LM3SUart {
    fn read(&self) -> u8 {
        let mut read_wait = 0;
        while self.uart0.fr.read().bits() != 0 {
            if read_wait == 1024 {
                return 0;
            }

            unsafe { core::arch::asm!("nop"); }
            read_wait += 1;
        }

        self.uart0.dr.read().bits() as u8
    }

    fn write(&self, byte: u8) {
        unsafe {
            self.uart0.dr.write(|w| {
                w.bits(byte as u32)
            });
        }
    }
}

impl Write for LM3SUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write(*b);
        }

        Ok(())
    }
}

impl Led for LM3SUart {}

impl Timer for LM3SUart {}

impl Driver for LM3SUart {
    fn driver_name(&self) -> &'static str {
        "LM3S6965 UART"
    }

    fn name(&self) -> &'static str {
        names::SERIAL
    }

    fn init(&mut self) -> DriverResult {
        unsafe {
            let peripherals = Peripherals::steal();
            let mut systcl = peripherals.SYSCTL;

            systcl.rcgcuart.write(|w| {
                w.bits(0x01)
            });
            systcl.rcgcgpio.write(|w| {
                w.bits(0x01)
            });

            peripherals.GPIO_PORTA.afsel.modify(|r, w| {
                w.bits(0x03)
            });

            peripherals.GPIO_PORTA.pctl.modify(|r, w| {
                w.bits((r.bits() & 0xFFFFFF00) + 0x00000011)
            });

            peripherals.GPIO_PORTA.den.write(|w| {
                w.bits(0x03)
            });

            self.uart0.ibrd.modify(|r, w| {
                w.bits(8)
            });

            self.uart0.ctl.write(|w| {
                w.bits(0x01)
            });

            self.uart0.fbrd.modify(|r, w| {
                w.bits(44)
            });

            self.uart0.lcrh.write(|w| {
                w.bits(0x70)
            });

            self.uart0.ctl.write(|w| {
                w.bits(0x301)
            });


            for _ in 0..150 {
                core::arch::asm!("nop");
            }

            //cortex_m::interrupt::disable();
        }

        if self.write_fmt(format_args!("{}", "\0\0\0\0\0\0\0\0")).is_ok() {
            Ok(())
        } else { Err("Failed to initialize UART") }
    }
}

gen_simpleuart!();

unsafe fn lm3s_serial_init() -> u8 {
    //KERNEL_SIMPLEUART.set_addrs(UART0_ADDR as *mut u8, UART0_ADDR as *mut u8);
    //DEVICE_DRIVERS.add_driver(&mut KERNEL_SIMPLEUART as &mut dyn Driver);

    let mut lm3s_uart = LM3SUart::new();
    let init_ret = lm3s_uart.init();

    if init_ret.is_err() {
        lm3s_uart.write_fmt(format_args!("{}", "failed\n"));
    } else { lm3s_uart.write_fmt(format_args!("{}", "success\n")); }

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, lm3s_serial_init);
