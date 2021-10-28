use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    hprintln!("File: {}, line: {} | ARM kernel panicked", info.location().unwrap().file(), info.location().unwrap().line());
    loop {  }
}
