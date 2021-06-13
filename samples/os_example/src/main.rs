#![no_std]
#![no_main]

mod required;

#[macro_use] extern crate aarch64_novusk;

#[macro_use]
extern crate libnu;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!("x86_64 OS example!");
    loop {  }
}
