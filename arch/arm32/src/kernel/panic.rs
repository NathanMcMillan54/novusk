use crate::arm32_printk;
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    arm32_printk!("\nARM32 kernel panicked:\n");
    arm32_printk!("Message: {:?}\n", info.message().unwrap());
    arm32_printk!("Location: {:?}:{:?}\n", info.location().unwrap().file(), info.location().unwrap().line());

    loop {    }
}
