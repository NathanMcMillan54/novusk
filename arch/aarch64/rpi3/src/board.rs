use rpi::led::Led;

pub fn act_init() {
    let led = Led::new();
    led.init();
    led.on();
    led.off();
}

#[no_mangle]
pub unsafe extern "C" fn rpi3_blink() {
    act_init();
}
