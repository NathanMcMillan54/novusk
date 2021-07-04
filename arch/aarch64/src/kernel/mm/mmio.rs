use crate::kernel::device::*;

pub struct Mmio;

impl Mmio {
    pub fn mmio(&self) -> Self {
        Self
    }

    pub unsafe fn mmio_init(&self, board: Board) {
        match board {
            Board::None =>
                MMIO_BASE = 0x0 as *mut u8,
            Board::Virt =>
                MMIO_BASE = 0x0a00_0000 as *mut u8,
            Board::RPi3 =>
                MMIO_BASE = 0x3f00_0000 as *mut u8,
            Board::RPi4 =>
                MMIO_BASE = 0xfe00_0000 as *mut u8,
            _ =>
                return
        }
    }
}
