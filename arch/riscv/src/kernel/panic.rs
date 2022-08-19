use core::panic::PanicInfo;
use crate::include::asm::wfi;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    early_printk!("\n\nKernel panicked:\n");

    early_printk!("    Message: {}\n", info.message().unwrap());
    early_printk!("    Location: {}\n", info.location().unwrap());

    unsafe { wfi(); }
}
