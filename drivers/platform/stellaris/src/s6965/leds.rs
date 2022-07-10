use asminc::nop;
use core::intrinsics::volatile_store;
use super::gpio::GPIO_PDOR;

pub fn led_on() {
    unsafe { volatile_store(GPIO_PDOR, 0x20); }
}

pub fn led_off() {
    unsafe { volatile_store(GPIO_PDOR, 0x0); }
}

#[no_mangle]
pub extern "C" fn device_indicate_panic() {
    led_on();

    for _ in 0..10000 {
        unsafe { nop(); }
    }

    led_off();
}
