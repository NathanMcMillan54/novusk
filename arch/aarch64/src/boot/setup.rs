use crate::drivers::device::DEVICE_INFO;
use arm_lib::include::asm::{wfe};
use super::{device};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    device::device_init();
    kinfo!("Device initialized | Machine: {} Manufacture: {} CPU: {}", DEVICE_INFO.board, DEVICE_INFO.manufacture, DEVICE_INFO.cpu);

    wfe()
}
