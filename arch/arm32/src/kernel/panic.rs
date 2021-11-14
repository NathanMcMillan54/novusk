use crate::arm32_printk;
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    arm32_printk!("\nFile: {}, line: {} | ARM kernel panicked\n", location.file(), location.line());
    arm32_printk!("{}\n", info.message().unwrap());
    loop {  }
}
