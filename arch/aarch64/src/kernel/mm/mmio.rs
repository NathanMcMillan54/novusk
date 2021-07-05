use crate::kernel::device::*;
use core::intrinsics::volatile_set_memory;
use core::ptr::read_volatile;

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

    pub unsafe fn mmio_write(&self, reg: u8, data: usize) {
        // This is the closest rust can get to c's volatile
        // This function is based off of https://wiki.osdev.org/ARM_RaspberryPi_Tutorial_C in the mmio_write function
        volatile_set_memory(MMIO_BASE, reg, data);
    }

    pub unsafe fn mmio_read(&self, reg: u8) {
        // TODO: Figure out how to return a C volatile
    }
}
