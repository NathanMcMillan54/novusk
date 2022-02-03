use core::ptr::write_volatile;
use rpi::common::{MMIO_BASE, UART_OFFSET};
use setup::{BootSetup, SetupReturn};

pub struct Aarch64Boot;

impl Aarch64Boot {
    pub fn new() -> Self {
        return Aarch64Boot;
    }

    pub fn setup(&self) {
        let early_cpu = self.early_cpu_init();

        if early_cpu.0.is_err() {
            panic!("{}", early_cpu.1);
        }

        if early_cpu.0.is_ok() {
            crate::early_printk!("{}", early_cpu.1);
        }
    }
}

impl BootSetup for Aarch64Boot {
    fn early_cpu_init(&self) -> SetupReturn {
        return (Ok(()), "Success");
    }
}
