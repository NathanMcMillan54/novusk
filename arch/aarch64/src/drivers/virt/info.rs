use super::uart;

pub const BOARD_NAME: &'static str = "QEMU virt";
pub const BOARD_MANUFACTURER: &'static str = "Qemu";
pub const CPU: &'static str = "Unknown";
pub const UART0: *mut u8 = 0x0900_0000 as *mut u8;
pub const MAIN_KERNEL: bool = false;
pub const ARCH_KERNEL: bool = false;

