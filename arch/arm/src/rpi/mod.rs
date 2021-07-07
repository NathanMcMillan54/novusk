#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("aarch64_start.S"));

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn aarch64_rpi_setup() -> ! {
    loop { asm!("wfe"); }
}
