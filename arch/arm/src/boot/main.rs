use novuskinc::kernel::start_kernel;
use crate::liba32::libdif::set_dif;
use super::setup::ArmBootSetup;

unsafe fn arm_boot_setup() {
    set_dif();

    let boot_setup = ArmBootSetup::new();
    boot_setup.setup();
}

unsafe fn arm_main() {
    arm_boot_setup();

    early_printk!("Starting kernel...\n");
    start_kernel();
}

#[cfg(not(feature = "cortex_m"))]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    arm_main();
    panic!("Kernel ended");
}

#[cfg(feature = "cortex_m")]
#[entry]
unsafe fn cm_boot_main() -> ! {
    arm_main();
    panic!("Kernel ended");
}
