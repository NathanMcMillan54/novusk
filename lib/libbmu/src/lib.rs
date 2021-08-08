#![no_std]

#[macro_use] extern crate printk;

pub use libc::memory as mem;

pub mod gpio;
pub use gpio::Gpio;
pub mod time;
pub use time::Time;

pub unsafe fn bmu_init() {
    printk!("\nStarting bare metal userspace...");
    extern "C" { fn kernel_main(); }
    kernel_main();
}
