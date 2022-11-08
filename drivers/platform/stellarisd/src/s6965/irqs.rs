use cortex_m::interrupt::{InterruptNumber, Nr};
use cortex_m::peripheral::NVIC;
use novuskinc::irq::*;
use novuskinc::kernel::types::KernelFunctionName;
use tm4c123x::Interrupt;

unsafe fn add_s6965_irqs() {
    add_irq(IrqHandler {
        irqn: Interrupt::TIMER0A.nr() as i16,
        irqh: TIMER0A,
    });
}

unsafe fn enable_s6965_irqs() {
    NVIC::unmask(Interrupt::TIMER0A);
}

unsafe fn s6965_irqs_init() -> u8 {
    add_s6965_irqs();
    enable_s6965_irqs();

    0
}

define_kernel_function!(KernelFunctionName::device_specific_irqs_init, -> u8, s6965_irqs_init);

// lm3s6965 IRQs
// -----

// TIMER0A
#[no_mangle]
pub unsafe extern "C" fn TIMER0A() -> i16 {
    IRQH_SUCCESS
}
