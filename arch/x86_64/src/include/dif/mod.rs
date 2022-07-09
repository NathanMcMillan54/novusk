use dif::Dif;
use novuskinc::power::{set_power_mode, PM_SHUTDOWN};
use crate::dif::DIF_FILE;
use crate::kernel::power::shutdown;

#[no_mangle]
pub static mut DIF: Dif = Dif::empty();

pub unsafe fn init_dif() {
    DIF.set_and_parse(DIF_FILE);
}

#[no_mangle]
pub unsafe extern "C" fn check_dif_panic() {
    let shutdown_value = DIF.get("ShutdownOnPanic");

    if shutdown_value.1.parse::<bool>().unwrap_or(false) {
        // set_power_mode(PM_SHUTDOWN);
        shutdown();
    }
}
