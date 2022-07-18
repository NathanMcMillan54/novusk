use tm4c123x::{Interrupt, NVIC};

pub mod gpio;
pub mod irqs;
pub mod leds;
pub mod uart;

pub fn lm3s6965_init() {
    unsafe { gpio::gpio_init(); }
}
