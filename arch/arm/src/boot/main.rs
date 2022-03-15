use crate::include::dif::{dif_init, dif::DIF};
use crate::include::dif::dif::DIF_FILE;
use crate::kernel::cpu::info::CPUINFO;
use crate::kernel::{arm_kernel_init, setup_arm_kernel};
use super::setup::ArmBoot;

#[no_mangle]
pub unsafe extern "C" fn arm_boot_main() {
    dif_init();

    let arm_boot = ArmBoot::new();
    arm_boot.setup();


    #[cfg(target_arch = "arm")]
    super::setup::boot32::arm32_boot_setup();

    #[cfg(target_arch = "aarch64")]
    crate::bits64::arm64_boot_setup();

    setup_arm_kernel();

    #[cfg(target_arch = "arm")]
    arm_kernel_init();

    #[cfg(target_arch = "aarch64")]
    crate::bits64::arm64_kernel_init();

    panic!("{}:{} kernel ended", CPUINFO.architecture, CPUINFO.bits);
}
