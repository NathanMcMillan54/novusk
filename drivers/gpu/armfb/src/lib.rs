#![no_std]

#[cfg(target_arch = "aarch64")]
pub mod a64;

pub fn armfb_init() {
    #[cfg(target_arch = "aarch64")]
    a64::a64_fb_init();
}
