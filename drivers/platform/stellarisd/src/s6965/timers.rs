use tm4c123x::Peripherals;

pub fn set_timer() {
    let peripherals = unsafe { Peripherals::steal() };
}