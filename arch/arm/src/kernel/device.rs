use alloc::string::String;
use super::kernel::{arm32_printk};

pub static mut DEVICE: Board = Board::None;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Board {
    Nrf52840,
    Nrf52832,
    None,
}


unsafe fn set_device(board: Board) {
    DEVICE = board;
}

pub(crate) unsafe fn device_init() {
    kinfo!("Initializing device...");
    // Set device value
    #[cfg(feature = "nrf52840")]
    set_device(Board::Nrf52840);

    arm32_printk!("    Board: {:?}", DEVICE);

    // Initialize device
    #[cfg(feature = "nrf")]
    crate::nrf::nrf_init();
}
