pub mod gpio;
pub mod leds;
pub mod uart;

pub fn lm3s6965_init() {
    unsafe { gpio::gpio_init(); }
}
