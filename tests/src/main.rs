#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

extern crate libc;

use core::panic::PanicInfo;
use libc::exit;

pub mod log;

#[no_mangle]
pub extern "Rust" fn main() {
    test_log!("Starting tests...\n");

    unsafe { exit(0) }
}

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    test_log!("Tests panicked\n");

    exit(1);
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { loop {  } }

#[no_mangle]
pub extern "C" fn _Unwind_Resume() { loop {  } }
