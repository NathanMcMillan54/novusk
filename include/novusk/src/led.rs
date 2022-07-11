extern "C" {
    pub fn set_led_on();
    pub fn set_led_off();
}

/// A trait for onboard LEDs
pub trait Led {
    /// Turns the oboard LED on
    fn on(&mut self) {

    }

    /// Turns the onboard LED off
    fn off(&mut self) {

    }

    /// Returns the status of the LED, the status indicates on/off/working
    fn status(&self) -> u32 {
        0
    }
}

pub fn led_blink(wait: u32) {
    unsafe {
        set_led_on();

        for _ in 0..wait {  }

        set_led_off();
    }
}
