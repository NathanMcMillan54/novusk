use stm32f4xx_hal::pac::{interrupt, CorePeripherals, NVIC};

empty_interrupt!(EXTI0);
empty_interrupt!(EXTI1);
empty_interrupt!(EXTI2);
empty_interrupt!(EXTI3);
empty_interrupt!(EXTI4);
empty_interrupt!(PVD);
empty_interrupt!(I2C3_EV);
empty_interrupt!(I2C3_ER);
empty_interrupt!(USART6);
empty_interrupt!(DMA2_STREAM7);
empty_interrupt!(DMA2_STREAM6);
empty_interrupt!(DMA2_STREAM5);
empty_interrupt!(DMA2_STREAM4);
empty_interrupt!(OTG_FS);

pub unsafe fn setup_interrupts() {
    NVIC::unmask(interrupt::EXTI0);
}
