use super::{BOARD, setup};
use cortex_m_rt::entry;

extern "C" {
    fn board_name() -> &'static str;
}

#[entry]
fn main() -> ! {
    unsafe {
        BOARD = board_name();
        setup::setup();
    }
}
