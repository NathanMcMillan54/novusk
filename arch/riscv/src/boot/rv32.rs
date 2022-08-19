use setup::BootSetup;
use super::setup::RiscvBoot;
use novuskinc::kernel::start_kernel;

fn boot_setup() {
    let riscv_boot = RiscvBoot::new();

    riscv_boot.setup();
}

#[entry]
fn rv32_start() -> ! {
    boot_setup();

    unsafe { start_kernel(); }

    panic!("Kernel ended");
}
