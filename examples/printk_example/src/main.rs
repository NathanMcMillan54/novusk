#![no_std]
#![no_main]

use novusk::x86::x86_printk;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    x86_printk!("Hello world!");
    loop {  }
}
