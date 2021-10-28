#![no_std]

#[macro_use] extern crate novuskinc;
// use tm4c123x_hal::Peripherals;

// pub mod led;

pub mod board {

}

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    // if Peripherals::take().is_none() {
        // panic!("Can't find peripherals");
    // }

    return (Ok(()), "Stellaris");
}
