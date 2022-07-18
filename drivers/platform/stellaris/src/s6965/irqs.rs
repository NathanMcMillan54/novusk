use core::fmt::Write;
use cortex_m::interrupt::Nr;
use cortex_m::peripheral::{NVIC, Peripherals};
use tm4c123x::{Interrupt};
use hio::io::HioWriter;
use novuskinc::irq::{add_irq, IRQH_SUCCESS, IrqHandler};
use super::uart::S6965UART;

/// Adds the implemented IRQ handlers to the NVIC
pub unsafe fn enable_s6965_irqs() {
    NVIC::unmask(Interrupt::TIMER0A);
    NVIC::unmask(Interrupt::UART0);

    // core::arch::asm!("wfi");
}

pub unsafe fn add_s6965_irqs() {
    add_irq(IrqHandler {
        irqn: Interrupt::TIMER0A.nr() as i16,
        irqh: TIMER0A,
    });

    add_irq(IrqHandler {
        irqn: Interrupt::UART0.nr() as i16,
        irqh: UART0,
    });
}

#[no_mangle]
pub unsafe extern "C" fn device_specific_irqs_init() {
    enable_s6965_irqs();
    add_s6965_irqs();
}

/// UART0 IRQ
#[no_mangle]
pub unsafe extern "C" fn UART0() -> i16 {
    S6965UART.uart0_write_byte(S6965UART.uart0_read_byte());

    IRQH_SUCCESS
}

/// TIMER0A IRQ
#[no_mangle]
pub unsafe extern "C" fn TIMER0A() -> i16 {
    IRQH_SUCCESS
}
