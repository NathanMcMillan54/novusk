use core::arch::asm;
use super::idt::{idt_init, set_idt};
use novuskinc::irq::irqchip_init;

pub(crate) static mut IRQS: X64Irqs = X64Irqs::new();


pub struct X64Irqs {
    pub enabled: bool,
}

impl X64Irqs {
    pub const fn new() -> Self {
        return X64Irqs {
            enabled: false,
        };
    }

    pub fn enable(&mut self) {
        unsafe { asm!("sti"); }
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        unsafe { asm!("cli"); }
        self.enabled = false;
    }

    pub fn disable_and_halt(&mut self) {
        self.disable();
        unsafe { asm!("hlt"); }
    }
}

pub unsafe fn start_irq_setup() {
    set_idt();
    idt_init();
}

pub unsafe fn irq_init() {
    irqchip_init();
    IRQS.enable();
}

pub mod irqns {
    pub const IRQ_1: u8 = 32;
    pub const IRQ_2: u8 = 33;
}

pub mod pics {
    pub const PIC_OFFSET_1: u8 = 32;
    pub const PIC_OFFSET_2: u8 = 40;
}
