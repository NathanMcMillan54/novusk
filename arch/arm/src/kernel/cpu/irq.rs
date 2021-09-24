use crate::kernel::early_printk::arm32_printk;
use cortex_m_rt_macros::interrupt;
use super::interrupts::interrupt0;
use irq::scope;

#[allow(non_camel_case_types)]
enum interrupt {
    INT0,
}

scoped_interrupts! {
    enum Interrupts {
        INT0,
    }

    use #[interrupt];
}

pub fn irq_init() {
    handler!(int0 = || interrupt0());

    irq::scope(|scope| {
        scope.register(Interrupts::INT0, int0);
    })
}

pub async fn irq_handler() {
    handler!(int0 = || interrupt0());

    int0.invoke();
}
