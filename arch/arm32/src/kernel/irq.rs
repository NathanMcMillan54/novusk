use super::cpu::interrupts::int_init;

pub fn irq_init() {
    int_init();
}