#![no_std]
#![feature(asm)]

pub use memory;

pub mod gpio;
pub use gpio::Gpio;
pub mod macros;
pub mod time;
pub use time::Time;

pub unsafe fn bmu_init() {
    extern "C" { fn kernel_main(); }
    kernel_main();
}
