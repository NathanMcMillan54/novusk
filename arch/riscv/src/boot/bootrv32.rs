use super::die;
use crate::riscv_printk;
use crate::kernel::board::Board;

#[entry]
fn main() -> ! {
    let mut board = Board;
    board.board_init();

    riscv_printk!("Starting kernel...");

    unsafe { die(); }
}
