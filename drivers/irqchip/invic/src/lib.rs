#![no_std]

#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate novuskinc;

use core::arch::asm;
use cortex_m::peripheral::{Peripherals, syst::SystClkSource};
use novuskinc::kernel::{types::KernelFunctionName};

pub mod handlers;

use handlers::COUNTED_SYSTICKS;

unsafe fn nvic_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::irqchip_init, -> u8, nvic_init);

unsafe fn nvic_early_setup() -> u8 {
    let cpu_peripherals = Peripherals::steal();
    let mut syst = cpu_peripherals.SYST;

    // Setup the SysTick interrupt
    // This will be used to check if interrupts are working
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();

    syst.enable_interrupt();

    asm!("wfi");

    if COUNTED_SYSTICKS != 1 {
        panic!("SysTick interrupt \"test\" failed");
    }

    0
}

define_kernel_function!(KernelFunctionName::irqchip_setup, -> u8, nvic_early_setup);
