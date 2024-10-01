#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}
