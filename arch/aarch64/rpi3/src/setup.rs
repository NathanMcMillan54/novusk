use arm_lib::include::asm::wfe;
use crate::board::act_init;
use rpi::led::Led;
use crate::print::early_write_byte;

extern "C" {
    fn aarch64_kernel_init();
    fn kernel_main() -> !;
    fn sleepm(mil: i32);
}

pub unsafe fn rpi_setup() -> ! {
    act_init();
    blink_ok();
    early_write_byte(69);
    aarch64_kernel_init();
    kernel_main();
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
