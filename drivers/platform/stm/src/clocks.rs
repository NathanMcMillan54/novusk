use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::pac::Peripherals;

pub fn setup_clocks() {
    let peripherals = unsafe { Peripherals::steal() };
    let clocks = peripherals.RCC.constrain();

    clocks.cfgr.sysclk(168.MHz()).freeze();
}