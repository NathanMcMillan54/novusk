use crate::mm::mmio::{mmio_init, MMIO_ADDRESS};

pub unsafe fn uart_init() {
    mmio_init();
}
