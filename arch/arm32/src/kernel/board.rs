use device::Board;

#[no_mangle]
pub static mut BOARD: Board = Board::empty();
