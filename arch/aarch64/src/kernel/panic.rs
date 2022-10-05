use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    printk!("\nKernel panicked:\n");
    printk!("   Location: {}:{}\n", location.file(), location.line());
    printk!("   Message: {}\n", info.message().unwrap());

    loop { }
}
