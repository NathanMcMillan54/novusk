// The Novusk kernel panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    loop {  }
}
