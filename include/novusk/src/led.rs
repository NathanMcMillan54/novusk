extern "C" {
    /// Turns on an led for ``sleep`` seconds or CPU cycles.
    pub fn led_blink(sleep: usize);
}
