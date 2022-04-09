use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(info: &PanicInfo) -> ! {
    crate::early_printk!("\nKernel panicked\n");

    let location = info.location().unwrap();

    crate::early_printk!("Location: In {} at line {}\n", location.file(), location.line());
    crate::early_printk!("Message: {}\n", info.message().unwrap());

    loop { asm!("wfi"); }
}
