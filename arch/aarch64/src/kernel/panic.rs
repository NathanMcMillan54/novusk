use crate::aarch64_printk;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    aarch64_printk!("\nKernel panicked:\n");
    aarch64_printk!("   Location: {}:{}\n", location.file(), location.line());
    aarch64_printk!("   Message: {}\n", info.message().unwrap());

    loop { asm!("wfi"); }
}
