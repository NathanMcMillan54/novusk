use core::panic::PanicInfo;
use crate::die;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    die();
}
