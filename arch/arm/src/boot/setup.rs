use crate::akernel::board::board_name;
use crate::akernel::init;
use drivers::arm::device::esp32::esp32_init;

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    if board_name() == "esp32" {
        esp32_init();
    } else {
        loop { asm!("wfe"); }
    }
    init::init()
}
