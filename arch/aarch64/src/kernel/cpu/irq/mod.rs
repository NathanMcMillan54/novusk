use core::arch::asm;

pub mod handlers;

pub(crate) static mut AARCH64_IRQS: Aarch64Irqs = Aarch64Irqs::new();

pub struct Aarch64Irqs {
    pub enabled: bool,
}

impl Aarch64Irqs {
    pub const fn new() -> Self {
        return Aarch64Irqs {
            enabled: false,
        };
    }

    pub unsafe fn enable(&mut self) {
        asm!("msr daifclr, #2");
        self.enabled = true;
    }

    pub unsafe fn disable(&mut self) {
        asm!("msr daifset, #2");
        self.enabled = false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_irq_setup() {
    AARCH64_IRQS.disable();
}
