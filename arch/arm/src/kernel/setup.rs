use super::early_printk::EARLYPRINTK;

pub fn setup_kernel() {
    EARLYPRINTK.lock().write_bytes(b"Setting up kernel...\n");
}