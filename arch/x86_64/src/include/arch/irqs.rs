#[no_mangle]
pub extern "C" fn disable_irqs() {
    x86_64::instructions::interrupts::disable();
}

#[no_mangle]
pub extern "C" fn enable_irqs() {
    x86_64::instructions::interrupts::enable();
}

#[no_mangle]
pub extern "C" fn irqs_enabled() -> u8 {
    x86_64::instructions::interrupts::are_enabled() as u8
}
