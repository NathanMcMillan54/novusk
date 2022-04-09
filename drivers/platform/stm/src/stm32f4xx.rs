use core::arch::asm;
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::Clocks;

pub(crate) fn get_clocks() -> Clocks {
    let peripherals = unsafe { Peripherals::steal() };

    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(16.MHz())
        .require_pll48clk()
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    return clocks;
}

pub fn setup_stm32f4xx() {
    if Peripherals::take().is_none() {
        panic!("Can't find device peripherals");
    }

    let clocks = get_clocks();

    if !clocks.is_pll48clk_valid() {
        panic!("Failed to setup clocks");
    }
}

pub fn setup_stm32f407() {
    setup_stm32f4xx();
}

pub fn finish_stm32f4xx_setup() {
    let peripherals = unsafe { Peripherals::steal() };

    let gpioa = peripherals.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    // Blink to show nothing went wrong
    for _ in 0..1000 {
        led.set_high();

        for _ in 0..1000 {
            unsafe { asm!("nop"); }
        }

        led.set_low();

        for _ in 0..1000 {
            unsafe { asm!("nop"); }
        }
    }
}
