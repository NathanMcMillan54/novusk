use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    loop { }
}
