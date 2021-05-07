use arm_lib::include::asm::wfe;
use crate::board::act_init;
use rpi::led::Led;

extern "C" {
    fn aarch64_kernel_init();
}

pub unsafe fn rpi_setup() -> ! {
    act_init();
    blink_ok();
    aarch64_kernel_init();
    wfe()
}

fn blink_ok() {
    let led = Led::new();
    led.init();
    // "O"
    led.on(); // -
    // Sleep 0.2
    led.off();
    led.on(); // -
    // Sleep 0.2
    led.off();
    led.on(); // -
    // Sleep 0.2

    led.off();
    // Sleep 0.4

    // "K"
    led.on(); // -
    // Sleep 0.2
    led.off();
    led.on(); // .
    // Sleep 0.1
    led.off();
    led.on(); // -
}
