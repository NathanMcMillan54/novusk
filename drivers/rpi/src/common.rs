pub mod bases {
    #[cfg(any(feature = "rpi3", feature = "rpi2"))]
    pub const MMIO_BASE: u32 = 0x3F00_0000;

    // This is common for all boards
    pub const GPIO_BASE: u32 = MMIO_BASE + 0x0020_0000;
    pub const UART_OFFSET: u32 = 0x0020_1000;
    pub const PL011_UART_START: u32 = MMIO_BASE + UART_OFFSET;
}

pub use bases::*;
