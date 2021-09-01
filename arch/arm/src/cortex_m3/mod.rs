use crate::kernel::early_printk::EARLYPRINTK;

pub mod hio;
pub use hio::hio_write_bytes;

pub fn cortex_m3_init() {
    EARLYPRINTK.lock().init("hio");
}
