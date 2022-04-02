extern "C" {
    pub fn set_led_on();
    pub fn set_led_off();
}

pub fn led_blink(wait: u32) {
    unsafe {
        set_led_on();

        for _ in 0..wait {  }

        set_led_off();
    }
}
