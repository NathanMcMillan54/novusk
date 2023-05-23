use cortex_m::interrupt::InterruptNumber;

#[cfg(any(feature = "s6965", feature = "s811"))]
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub(crate) enum LM3SInts {
    GPIOA_INT = 0,
    GPIOB_INT = 1,
    GPIOC_INT = 2,
    GPIOD_INT = 3,
    GPIOE_INT = 4,
    UART0_INT = 5,
    UART1_INT = 6,
    TIMER0A_INT = 14,
}

#[cfg(any(feature = "s6965", feature = "s811"))]
unsafe impl InterruptNumber for LM3SInts {
    fn number(self) -> u16 {
        self as u16
    }
}
