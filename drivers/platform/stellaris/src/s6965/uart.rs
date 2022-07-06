use core::ops::Deref;
use core::fmt::Write;
use core::ptr::write_volatile;
use tm4c123x::{Peripherals, UART0};
use tm4c123x_hal::gpio::{AF1, GpioExt};
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl::SysctlExt;
use tm4c123x_hal::time::U32Ext;

pub fn uart_init() {
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

    uart0.write_fmt(format_args!("{}", "Test"));
}