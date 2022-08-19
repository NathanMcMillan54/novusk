use kinfo::InfoDisplay;
use kinfo::status::KStatus;
use novuskinc::kernel::{arch_prepare_init, setup_arch};
use setup::arch::ArchKernelSetup;
use setup::SetupReturn;

pub(crate) static mut RISCV_KERNEL: RiscVKernel = RiscVKernel;

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    setup_arch();

    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        message1: "Finished RISCV kernel setup",
        messages: Some(&[
            "Initialized IRQs",
            "Initialized device",
        ])
    });

    arch_prepare_init();
}

pub(crate) struct RiscVKernel;

impl RiscVKernel {
    pub fn new() -> Self {
        return RiscVKernel;
    }

    pub fn setup(&self) {
        let irq_ret = self.irq_init();
        let mem_ret = self.memory_setup();

        let mut dev_ret: SetupReturn;
        //let mut sys_ret: SetupReturn;

        unsafe {
            dev_ret = self.device_init();
            //sys_ret = self.sys_setup();
        }

        early_printk!("Finished RISCV kernel setup\n");
        early_printk!("Running on: {}\n", dev_ret.1);
        early_printk!("{}\n{}\n", irq_ret.1, mem_ret.1);
    }
}
