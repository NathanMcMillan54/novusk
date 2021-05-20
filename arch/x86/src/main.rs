#![no_std]
#![no_main]

extern crate x86_novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! { loop {  } }

#[no_mangle]
pub extern "C" fn kernel_info() -> bool { return true; }
