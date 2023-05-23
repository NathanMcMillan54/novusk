use cortex_m::peripheral::NVIC;
use novuskinc::kernel::types::KernelFunctionName;
use crate::interrupts::nums::LM3SInts;

unsafe fn enable_interrupts() {
    NVIC::unmask(LM3SInts::UART0_INT);
    NVIC::unmask(LM3SInts::TIMER0A_INT);
}

unsafe fn early_lm3s_init() -> u8 {
    enable_interrupts();

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_lm3s_init);
