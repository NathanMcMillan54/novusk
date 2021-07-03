#![no_std]
#![feature(global_asm)]

#[cfg(target_arch = "x86_64")]
mod ex1;

#[cfg(target_arch = "x86_64")]
pub unsafe fn x64_start() {
    ex1::ex1_init();
}

#[cfg(target_arch = "x86")]
pub unsafe fn x86_start() {

}

