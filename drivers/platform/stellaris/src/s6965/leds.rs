use core::intrinsics::volatile_store;
use super::gpio::GPIO_PDOR;

pub fn led_on() {
    unsafe { volatile_store(GPIO_PDOR, 0x20); }
}

pub fn led_off() {
    unsafe { volatile_store(GPIO_PDOR, 0x0); }
}
