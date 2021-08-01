use super::die;
use crate::kernel::board::Board;

use hifive1::sprintln;

#[entry]
fn main() -> ! {
    let mut board = Board;
    board.board_init();

    sprintln!("Starting kernel...");
    loop {  }
}
