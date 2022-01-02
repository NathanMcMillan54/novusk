use setup::BootSetup;
use super::setup::RiscvBoot;
use crate::kernel::setup::setup_riscv_kernel;
use crate::rv_printk;

fn setup() {
    let riscv_boot = RiscvBoot::new();

    riscv_boot.setup();
}

#[entry]
fn rv32_start() -> ! {
    setup();

    setup_riscv_kernel();
    //kinfo!("Kernel initialized\n");

    panic!("Kernel ended");
}
