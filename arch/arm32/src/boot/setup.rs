use crate::drivers::stm32::stm32f4::stm32f4xx_init;
use crate::include::asm::wfe;
use super::BOARD;

unsafe fn die() -> ! {
    loop { wfe(); }
}

pub unsafe fn setup() -> ! {
    // TODO: Simplify this
    if BOARD == "STM32F4" {
        stm32f4xx_init();
    } else {
        die();
    }

    wfe();
}
