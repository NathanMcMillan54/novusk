use stm32f4xx_hal::pac::interrupt;
use stm32f4xx_hal::pac::NVIC;

pub(crate) const STM32F4XX_INTERRUPTS: &[interrupt; 1] = &[interrupt::TIM7];

cm_empty_interrupt!(LCD_TFT);
cm_empty_interrupt!(LCD_TFT_1);
cm_empty_interrupt!(FPU);
cm_empty_interrupt!(HASH_RNG);
cm_empty_interrupt!(CRYP);
cm_empty_interrupt!(DCMI);
cm_empty_interrupt!(EXTI15_10);
cm_empty_interrupt!(OTG_HS);
cm_empty_interrupt!(OTG_HS_WKUP);
cm_empty_interrupt!(OTG_HS_EP1_IN);
cm_empty_interrupt!(OTG_HS_EP1_OUT);
cm_empty_interrupt!(CAN2_SCE);
cm_empty_interrupt!(CAN2_RX1);
cm_empty_interrupt!(CAN2_RX0);
cm_empty_interrupt!(CAN2_TX);
cm_empty_interrupt!(CAN1_SCE);
cm_empty_interrupt!(CAN1_RX1);
cm_empty_interrupt!(CAN1_RX0);
cm_empty_interrupt!(CAN1_TX);
cm_empty_interrupt!(ETH_WKUP);
cm_empty_interrupt!(ETH);
cm_empty_interrupt!(TIM7);
cm_empty_interrupt!(TIM8_CC);
cm_empty_interrupt!(TIM8_TRG_COM_TIM14);
cm_empty_interrupt!(TIM8_UP_TIM13);
cm_empty_interrupt!(TIM8_BRK_TIM12);
cm_empty_interrupt!(TIM6_DAC);
cm_empty_interrupt!(UART5);
cm_empty_interrupt!(UART4);
cm_empty_interrupt!(USART3);
cm_empty_interrupt!(USART2);
cm_empty_interrupt!(SDIO);
cm_empty_interrupt!(FSMC);
cm_empty_interrupt!(OTG_FS_WKUP);
cm_empty_interrupt!(ADC);
cm_empty_interrupt!(RCC);
cm_empty_interrupt!(WWDG);

pub unsafe fn stm32f4xx_specific_interrupts_setup() {
    for int in 0..STM32F4XX_INTERRUPTS.len() {
        NVIC::unmask(STM32F4XX_INTERRUPTS[int]);
    }
}
