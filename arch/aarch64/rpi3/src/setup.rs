use arm_lib::include::asm::wfe;
use crate::board::act_init;
use rpi::led::Led;

extern "C" {
    fn aarch64_kernel_init();
    fn sleepm(mil: i32);
}

pub unsafe fn rpi_setup() -> ! {
    act_init();
    blink_ok();
    aarch64_kernel_init();
    wfe()
}

unsafe fn blink_ok() {
    let led = Led::new();
    led.init();
    // "O"
    led.on(); // -
    sleepm(200);
    led.off();
    led.on(); // -
    sleepm(200);
    led.off();
    led.on(); // -
    sleepm(200);

    led.off();
    sleepm(400);

    // "K"
    led.on(); // -
    sleepm(200);
    led.off();
    led.on(); // .
    sleepm(100);
    led.off();
    led.on(); // -
    led.off();
}
