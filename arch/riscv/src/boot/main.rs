use super::die;
use crate::kernel::{board::Board, device::Device, riscv::riscv_init};
use crate::riscv_printk;

#[no_mangle]
pub unsafe extern "C" fn start_riscv_kernel() -> ! {
    let mut board = Board;
    board.device_init();

    riscv_printk!("Starting kernel...\n");

    kinfo!("Device initialized");
    riscv_printk!("    Running on: {}", board.name());

    riscv_init();
    die();
}
