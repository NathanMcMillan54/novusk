use crate::early_printk;
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
unsafe fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    early_printk!("\nKernel panicked\n");

    early_printk!("    Location: {}:{}\n", location.file(), location.line());
    early_printk!("    Message: {}\n", info.message().unwrap());

    loop { asm!("wfi"); }
}
