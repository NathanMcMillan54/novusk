use esp32_fw::EspLed;
use esp32_fw::led::Led;
use esp32_hal::hal::digital::v2::OutputPin;

pub fn blink_init() {
    esp32_blink();
}

#[no_mangle]
pub extern "C" fn esp32_blink() {
    let mut blinker = EspLed::new();
    blinker.set_high();
    blinker.set_low();
}
