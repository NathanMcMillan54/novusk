use crate::kernel::early_printk::EARLYPRINTK;

pub fn cortex_m4_init() {
    EARLYPRINTK.lock().init("UART");
}
