#![no_std]
#![no_main]

extern crate novusk;
#[macro_use]
use novusk::kernel::printk;

use core::panic::PanicInfo;

mod required;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    printk!("Example OS!");
    loop {    }
}

/* #[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
*/
