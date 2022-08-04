use super::irqs::irq_error;
use crate::device::{interrupt, Interrupt};

// Interrupts setup/run

pub unsafe fn enable_common_interrupts() {
    use cortex_m::{Peripherals, peripheral::NVIC};

    let cpu_peripherals = Peripherals::take().unwrap();

    let mut nvic = cpu_peripherals.NVIC;
    let syst = cpu_peripherals.SYST;

    NVIC::unmask(Interrupt::EXTI0);
}

pub unsafe fn run_common_interrupts() {
    use stm32f4xx_hal::stm32::NVIC;


    NVIC::pend(Interrupt::EXTI0);
}

// STM specific interrupts
#[interrupt]
fn EXTI0() {

}

#[interrupt]
fn USART1() {

}

#[interrupt]
fn USART2() {
    use crate::device::prelude::{_embedded_hal_serial_Read, _embedded_hal_serial_Write};
    use crate::io::get_serial;

    let mut usart2 = get_serial();

    if !usart2.is_tx_empty() {
        usart2.unlisten();

        usart2.write(0);
    } else { irq_error("USART2", "TX pin is empty"); }

    usart2.listen();
}
