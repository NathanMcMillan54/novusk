#![no_std]
#![no_main]

pub mod required;

#[macro_use] extern crate x86_novusk;

#[macro_use] extern crate libnu;

// android.c
include!("android.rs");

/*
extern "C" {
    fn android_start() -> !;
    fn android_version() -> i32;
}
*/

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    println!("Novusk kernel finished");
    println!("Starting Android...");
    android_start()
}
