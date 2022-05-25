use crate::include::dif::set_dif;
use super::setup::ArmBoot;

#[no_mangle]
pub unsafe extern "C" fn arm_boot_setup() {
    set_dif();

    let boot = ArmBoot::new();
    boot.setup();
}

#[cfg(not(feature = "cortex_m"))]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    arm_boot_setup();
    panic!("ARM kernel ended");
}
