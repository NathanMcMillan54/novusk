use crate::kernel::die;
use core::panic::PanicInfo;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    die();
}
