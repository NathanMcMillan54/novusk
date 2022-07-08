use tm4c123x::{Interrupt, NVIC};
use super::uart::S6965UART;

/// Adds the implemented IRQ handlers to the NVIC
pub unsafe fn enable_s6965_irqs() {
    NVIC::unmask(Interrupt::UART0);
    NVIC::unmask(Interrupt::TIMER0A);
}

/// UART0 IRQ
#[no_mangle]
pub unsafe extern "C" fn uart0_isr() {
    S6965UART.uart0_write_byte(S6965UART.uart0_read_byte());
}

#[no_mangle]
pub unsafe extern "C" fn timer0a_isr() {

}

#[no_mangle]
pub unsafe extern "C" fn device_specific_irqs_init() {
    enable_s6965_irqs();
}
