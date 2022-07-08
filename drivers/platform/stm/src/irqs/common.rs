use stm32f4xx_hal::pac::{interrupt, CorePeripherals, NVIC};

const COMMON_INTERRUPTS: &[interrupt; 5] = &[interrupt::EXTI0, interrupt::EXTI1, interrupt::EXTI2, interrupt::EXTI3, interrupt::EXTI4];

cm_empty_interrupt!(EXTI0);
cm_empty_interrupt!(EXTI1);
cm_empty_interrupt!(EXTI2);
cm_empty_interrupt!(EXTI3);
cm_empty_interrupt!(EXTI4);
cm_empty_interrupt!(PVD);
cm_empty_interrupt!(I2C3_EV);
cm_empty_interrupt!(I2C3_ER);
cm_empty_interrupt!(I2C2_EV);
cm_empty_interrupt!(I2C2_ER);
cm_empty_interrupt!(I2C1_EV);
cm_empty_interrupt!(I2C1_ER);
cm_empty_interrupt!(USART6);
cm_empty_interrupt!(USART1);
cm_empty_interrupt!(DMA2_STREAM7);
cm_empty_interrupt!(DMA2_STREAM6);
cm_empty_interrupt!(DMA2_STREAM5);
cm_empty_interrupt!(DMA2_STREAM4);
cm_empty_interrupt!(DMA2_STREAM3);
cm_empty_interrupt!(DMA2_STREAM2);
cm_empty_interrupt!(DMA2_STREAM1);
cm_empty_interrupt!(DMA2_STREAM0);
cm_empty_interrupt!(DMA1_STREAM7);
cm_empty_interrupt!(DMA1_STREAM6);
cm_empty_interrupt!(DMA1_STREAM5);
cm_empty_interrupt!(DMA1_STREAM4);
cm_empty_interrupt!(DMA1_STREAM3);
cm_empty_interrupt!(DMA1_STREAM2);
cm_empty_interrupt!(DMA1_STREAM1);
cm_empty_interrupt!(DMA1_STREAM0);
cm_empty_interrupt!(OTG_FS);
cm_empty_interrupt!(TIM5);
cm_empty_interrupt!(TIM4);
cm_empty_interrupt!(TIM3);
cm_empty_interrupt!(TIM2);
cm_empty_interrupt!(TIM1_CC);
cm_empty_interrupt!(TIM1_TRG_COM_TIM11);
cm_empty_interrupt!(TIM1_UP_TIM10);
cm_empty_interrupt!(TIM1_BRK_TIM9);
cm_empty_interrupt!(EXTI9_5);
cm_empty_interrupt!(SPI3);
cm_empty_interrupt!(SPI2);
cm_empty_interrupt!(SPI1);
cm_empty_interrupt!(RTC_ALARM);
cm_empty_interrupt!(RTC_WKUP);
cm_empty_interrupt!(TAMP_STAMP);

pub unsafe fn setup_interrupts() {
    for int in 0..COMMON_INTERRUPTS.len() {
        NVIC::unmask(COMMON_INTERRUPTS[int]);
    }
}
