use arm_lib::include::asm::{wfe};
use super::{device};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    device::device_init();
    wfe()
}
