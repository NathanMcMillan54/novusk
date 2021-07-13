use crate::kernel::device::DEVICE;

#[cfg(feature = "nrf")]
fn nrf_blink() {
    use crate::nrf::onboard::led::NrfLed;
    let mut led = unsafe { NrfLed::new(&mut NrfLed{
        board: DEVICE
    }) };

    led.blink();
}

#[no_mangle]
pub extern "C" fn blink() {
    #[cfg(feature = "nrf")]
    nrf_blink();
}
