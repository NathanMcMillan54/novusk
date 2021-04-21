pub mod board;
pub mod init;

pub fn board_name() -> &'static str {
    return board::BOARD;
}