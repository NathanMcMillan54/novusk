use core::panic::PanicInfo;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("hlt"); } }
}
