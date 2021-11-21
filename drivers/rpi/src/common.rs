pub mod bases {
    #[cfg(any(feature = "rpi3", feature = "rpi2"))]
    pub const MMIO_BASE: u32 = 0x3F00_0000;

    // This is common for all boards
    pub const GPIO_BASE: u32 = MMIO_BASE + 0x200000;
}

pub use bases::*;
