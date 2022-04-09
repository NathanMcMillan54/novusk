use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {

    loop { asm!("wfi"); }
}
