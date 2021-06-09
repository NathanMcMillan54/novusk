use crate::drivers::{drivers_init, blink};

extern "C" {
    fn app_init() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    blink::blink();
    blink::blink();
    dprintln!("Starting Xtensa Novusk kernel...");
    drivers_init();
    blink::blink();
    blink::blink();
    dprintln!("Xtnesa drivers initialized");

    // You Xtensa app
    app_init()
}
