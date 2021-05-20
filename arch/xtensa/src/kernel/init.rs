use crate::drivers::{drivers_init, blink};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    blink::blink();
    blink::blink();
    dprintln!("Starting Xtensa Novusk kernel...");
    drivers_init();
    blink::blink();
    blink::blink();
    dprintln!("Xtnesa drivers initialized");
}
