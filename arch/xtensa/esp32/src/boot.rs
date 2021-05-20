use super::{blink, drivers};

extern "C" {
    fn kernel_init();
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    blink::blink_init();
    dprintln!("Blink initialized");

    drivers::esp32_drivers_init();
    dprintln!("Esp32 drivers set");

    kernel_init();
    dprintln!("Novusk initialized");

    // TODO: Loop with Xtensa assembly
    loop {  }
}
