#[cfg(target_arch = "aarch64")]
#[no_mangle]
extern "C" {
    pub(self) fn aarch64_boot_start() -> !;
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub extern "C" fn arm_boot_start() -> ! {
    panic!("ARM boot finished");
}
