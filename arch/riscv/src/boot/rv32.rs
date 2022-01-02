use sifive::sprint;
use super::setup::RiscvBoot;

fn setup() {
    let riscv_boot = RiscvBoot::new();

    riscv_boot.setup();
}

#[entry]
fn rv32_start() -> ! {
    setup();

    sprint!("Test\n");

    loop {  }
}
