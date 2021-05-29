use libefi::{st, power::UefiPower};
use uefi::proto::console::text::Input;

#[no_mangle]
pub unsafe extern "C" fn input() -> &'static mut Input {
    // TODO: Get input working
    let input = st().as_ref().stdin();
    return input;
}

#[no_mangle]
pub unsafe extern "C" fn reboot() -> ! {
    let mut power = UefiPower;
    power.reboot()
}

#[no_mangle]
pub unsafe extern "C" fn shutdown() -> ! {
    let mut power = UefiPower;
    power.shutdown()
}
