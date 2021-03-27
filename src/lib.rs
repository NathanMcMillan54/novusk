#![no_std]

mod kernel;

use core::panic::{PanicInfo};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() -> ! {
    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
