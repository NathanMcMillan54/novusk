use cortex_m::interrupt::free;
use cortex_m::peripheral::NVIC;
use novuskinc::kernel::types::KernelFunctionName;
use tm4c123x_hal::Peripherals;
use tm4c123x_hal::prelude::SysctlExt;
use tm4c123x_hal::sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, Sysctl, SystemClock};
use crate::interrupts::nums::LM3SInts;

pub mod gpio;
pub mod uart;

unsafe fn enable_interrupts() {
    NVIC::unmask(LM3SInts::UART0_INT);
    NVIC::unmask(LM3SInts::TIMER0A_INT);
}

unsafe fn early_lm3s_init() -> u8 {
    //extern "C" {
      //  fn _early_print(args: core::fmt::Arguments);
    //}

    //_early_print(format_args!("{}", "\0"));

    super::setup_sys_clock();
    uart::early_serial_init();
    //gpio::setup_gpio();
    //panic!("here");

    //enable_interrupts();
    //uart::early_serial_init();

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_lm3s_init);
