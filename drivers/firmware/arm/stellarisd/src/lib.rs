#![no_std]

#[macro_use] extern crate novuskinc;
use tm4c123x_hal::Peripherals;

// pub mod led;

pub mod board {

}

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    cortex_m_semihosting::hprintln!("test");
    let mut error = false;

    if unsafe { Peripherals::take().is_none() } {
        error = true;
    } else { error = false; }

    if error {
        return (Err("Cannot find peripherals"), "Stellaris 6965");
    } else { return (Ok(()), "Stellaris 6965"); }
}
