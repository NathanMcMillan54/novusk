use core::panic::PanicInfo;
use crate::rv_printk;
use crate::include::asm::wfi;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    rv_printk!("\n\nKernel panicked:\n");

    rv_printk!("    Message: {}\n", info.message().unwrap());
    rv_printk!("    Location: {}\n", info.location().unwrap());

    unsafe { wfi(); }
}
