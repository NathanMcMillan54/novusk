use crate::nrf::nrf_init;
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
    // Set device value
    #[cfg(feature = "nrf52840")]
    set_device(Board::Nrf52840);

    // #[cfg(feature = "nrf52832")]
    // DEVICE = Board::Nrf52832;

    // Initialize device
    #[cfg(feature = "nrf")]
    nrf_init();

    kinfo!("Device initialized");
    arm32_printk!("    Board: {:?}", DEVICE);
}
