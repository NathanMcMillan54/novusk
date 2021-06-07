#![no_std]
#![no_main]

mod required;

#[cfg(target_arch = "x86_64")]
#[macro_use] extern crate x86_novusk;

#[macro_use]
extern crate libnu;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!("x86_64 OS example!");
    loop {  }
}
