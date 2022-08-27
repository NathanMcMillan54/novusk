use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    printk!("\nARM32 kernel panicked:\n");
    printk!("Message: {:?}\n", info.message().unwrap());
    printk!("Location: {:?}:{:?}\n", info.location().unwrap().file(), info.location().unwrap().line());

    loop {    }
}
