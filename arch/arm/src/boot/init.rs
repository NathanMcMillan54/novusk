use super::cpu;
use crate::include::asm::wfe;
use crate::kernel::{device, early_printk::EARLYPRINTK, setup};
use crate::mm::early_memory_init;

#[entry]
fn init() -> ! {
    unsafe {
        EARLYPRINTK.lock().init("hio");
        kinfo!("Early printk initialized");
        printk!("    Using hio");
        early_memory_init();
        cpu::cpu_init();
        device::device_init();
        setup::setup_kernel();
        panic!("Kernel ended");
    }
}
