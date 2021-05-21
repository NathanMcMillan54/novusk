#![no_std]
#![no_main]

extern crate x86_novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! { loop {  } }

#[no_mangle]
pub extern "C" fn kernel_info() -> bool { return true; }

#[no_mangle]
pub extern "C" fn application_type() -> &'static str { return "none"; }

// TODO: Make a color type
#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "cyan" }