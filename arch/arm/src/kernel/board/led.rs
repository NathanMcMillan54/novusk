use crate::kernel::device::DEVICE;

#[cfg(feature = "nrf")]
fn nrf_blink() {
    use crate::nrf::nrf_board::NrfLed;
    let mut led = NrfLed::new();

    led.blink();
}

#[no_mangle]
pub extern "C" fn blink() {
    nrf_blink();
}
