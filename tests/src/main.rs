#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate libc;
pub(crate) extern crate nmallocator;

use core::alloc::Layout;
use core::panic::PanicInfo;
use libc::exit;

pub mod log;

pub mod mm;

#[no_mangle]
pub extern "Rust" fn main() {
    test_log!("Starting tests...\n\n");

    mm::run_mm_tests();

    unsafe { exit(0); }
}

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    test_log!("\ntests panicked\n");
    test_log!("A test failed, exiting with code '1'");

    exit(1);
}

#[no_mangle]
pub extern "C" fn rust_oom() -> ! {
    test_log!("tests are OOM\n");
    panic!("");
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { loop {  } }

#[no_mangle]
pub extern "C" fn _Unwind_Resume() { loop {  } }
