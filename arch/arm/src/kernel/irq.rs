use crate::target::ints::enable_interrupts;
use novuskinc::irq::irqchip_init;

extern "C" {
    /* Should be called after irqchip_setup, triggers an exception to make sure irqchip_setup was
       successful */
    pub(crate) fn test_exception() -> u8;
}

pub unsafe fn irq_init() {
    enable_interrupts();
    irqchip_init();
}
