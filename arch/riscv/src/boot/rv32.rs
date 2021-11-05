use crate::kernel::setup::setup_riscv_kernel;
use riscv_rt::entry;

#[entry]
fn rv32_boot() -> ! {
    setup_riscv_kernel();

    loop {  }
}
