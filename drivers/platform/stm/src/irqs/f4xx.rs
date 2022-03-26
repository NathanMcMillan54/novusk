use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::pac::NVIC;

pub(crate) const STM32F4XX_INTERRUPTS: &[interrupt; 1] = &[interrupt::TIM7];

empty_interrupt!(LCD_TFT);
empty_interrupt!(LCD_TFT_1);
empty_interrupt!(FPU);
empty_interrupt!(HASH_RNG);
empty_interrupt!(CRYP);
empty_interrupt!(DCMI);
empty_interrupt!(EXTI15_10);
empty_interrupt!(OTG_HS);
empty_interrupt!(OTG_HS_WKUP);
empty_interrupt!(OTG_HS_EP1_IN);
empty_interrupt!(OTG_HS_EP1_OUT);
empty_interrupt!(CAN2_SCE);
empty_interrupt!(CAN2_RX1);
empty_interrupt!(CAN2_RX0);
empty_interrupt!(CAN2_TX);
empty_interrupt!(CAN1_SCE);
empty_interrupt!(CAN1_RX1);
empty_interrupt!(CAN1_RX0);
empty_interrupt!(CAN1_TX);
empty_interrupt!(ETH_WKUP);
empty_interrupt!(ETH);
empty_interrupt!(TIM7);
empty_interrupt!(TIM8_CC);
empty_interrupt!(TIM8_TRG_COM_TIM14);
empty_interrupt!(TIM8_UP_TIM13);
empty_interrupt!(TIM8_BRK_TIM12);
empty_interrupt!(TIM6_DAC);
empty_interrupt!(UART5);
empty_interrupt!(UART4);
empty_interrupt!(USART3);
empty_interrupt!(USART2);
empty_interrupt!(SDIO);
empty_interrupt!(FSMC);
empty_interrupt!(OTG_FS_WKUP);
empty_interrupt!(ADC);
empty_interrupt!(RCC);
empty_interrupt!(WWDG);

pub unsafe fn stm32f4xx_specific_interrupts_setup() {
    for int in 0..STM32F4XX_INTERRUPTS.len() {
        NVIC::unmask(STM32F4XX_INTERRUPTS[int]);
    }
}
