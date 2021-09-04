pub fn blink(sleep_time: usize) {
    extern "C" { fn led_blink(sleep: usize); }

    unsafe { led_blink(sleep_time); }
}
