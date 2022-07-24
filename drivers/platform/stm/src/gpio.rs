use stm32f4xx_hal::pac::Peripherals;

pub fn gpio_init() {
    let peripherals = unsafe { Peripherals::steal() };
    let rcc = peripherals.RCC;

    rcc.ahb1enr.modify(|r, w | {
        w.gpioaen().enabled();
        w.gpioben().enabled();
        w.gpiocen().enabled();
        w.gpioden().enabled()
    });
}