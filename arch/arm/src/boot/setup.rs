use crate::device::{BOARD, esp32::esp32_init};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    if BOARD == "esp32" {
        esp32_init();
    } else {
        loop { asm!("wfe"); }
    }
}
