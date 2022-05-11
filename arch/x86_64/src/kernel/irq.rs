use core::arch::asm;
use super::i8259::PIC_8259;
use super::idt::{idt_init, set_idt};

pub(crate) static mut IRQS: X64Irqs = X64Irqs::new();

pub const PIC_START: u8 = 32;
pub const PIC_OFFSETS: &[u8; 12] = &[0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88];

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
    PIC_8259.lock().initialize();
    IRQS.enable();
}

