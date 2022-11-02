use crate::liba32::libdif::set_dif;
use super::setup::ArmBootSetup;

unsafe fn arm_boot_setup() {
    set_dif();

    let boot_setup = ArmBootSetup::new();
    boot_setup.setup();
}

#[cfg(not(feature = "cortex_m"))]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    arm_boot_setup();
    panic!("Kernel ended");
}

#[cfg(feature = "cortex_m")]
#[entry]
unsafe fn cm_boot_main() -> ! {
    arm_boot_setup();
    panic!("Kernel ended");
}
