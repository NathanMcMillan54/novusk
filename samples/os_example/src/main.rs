#![no_std]
#![no_main]

mod required;

#[cfg(target_arch = "x86_64")]
#[macro_use] extern crate x86_novusk;
use x86_novusk::printk::printk;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    printk!("x86_64 OS example");

    loop {  }
}
