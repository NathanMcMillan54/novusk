use riscv::interrupt::{enable};

pub unsafe fn irq_init() {
    enable();
}
