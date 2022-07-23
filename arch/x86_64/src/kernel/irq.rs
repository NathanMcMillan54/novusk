use core::arch::asm;
use super::idt::{idt_init, set_idt};
use super::time::TIME_RATE;
use novuskinc::irq::irqchip_init;
use time::kernel::set_kernel_time_rate;
use crate::include::asm::irq::ARCH_IRQS;

pub unsafe fn start_irq_setup() {
    set_idt();
    idt_init();

    set_kernel_time_rate(TIME_RATE);
}

pub unsafe fn irq_init() {
    //super::i8259::PIC2859.lock().initialize();
    irqchip_init();
    ARCH_IRQS.enable();
}

pub mod irqns {
    pub const IRQ_1: u8 = 32;
    pub const IRQ_2: u8 = 33;
}

pub mod pics {
    pub const PIC_OFFSET_1: u8 = 32;
    pub const PIC_OFFSET_2: u8 = 40;
}
