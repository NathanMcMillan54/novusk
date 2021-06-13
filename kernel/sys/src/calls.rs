use core::str::from_utf8_unchecked;
use libefi::{st, power::UefiPower};
use printk::printk;
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

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8]) {
    printk!("{}", from_utf8_unchecked(bytes));
}
