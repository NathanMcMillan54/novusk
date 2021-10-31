use crate::arm32_printk;
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    arm32_printk!("File: {}, line: {} | ARM kernel panicked", location.file(), location.line());
    arm32_printk!("{}", info.message().unwrap());
    loop {  }
}
