use core::panic::PanicInfo;
use crate::include::asm::wfi;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    wfi();
}
