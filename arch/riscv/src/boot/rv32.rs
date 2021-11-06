use device::Device;
use crate::board::get_board;
use crate::kernel::setup::setup_riscv_kernel;
use riscv_rt::entry;

#[entry]
fn rv32_boot() -> ! {
    let mut board = get_board();
    board.serial_io_init();
    kinfo!("Early I/O initialized");

    setup_riscv_kernel();
    kinfo!("RISCV kernel initialized");

    panic!("Kernel ended, nothing to run");
}
