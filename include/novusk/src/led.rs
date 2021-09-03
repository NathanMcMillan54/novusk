#[macro_export]
macro_rules! define_led_blink_function {
    ($blink:ident) => {
        #[no_mangle]
        pub extern "C" fn led_blink(sleep: usize) {
            $blink(sleep);
        }
    };
}

pub fn blink(sleep_time: usize) {
    extern "C" { fn led_blink(sleep: usize); }

    unsafe { led_blink(sleep_time); }
}
