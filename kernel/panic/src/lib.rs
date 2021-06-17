#![no_std]

#[macro_use] extern crate macros;

use core::panic::PanicInfo;

mod exit;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    printk!("Kernel panic:");
    printk!("{:?}", _info);

    loop { exit::exit_panic(); }
}
