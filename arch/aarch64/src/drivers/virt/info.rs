use super::uart;

pub const BOARD_NAME: &'static str = "QEMU virt";
pub const BOARD_MANUFACTURER: &'static str = "none";
pub const CPU: &'static str = "Unknown";
pub const UART0: *mut u8 = uart::UART;
pub const MAIN_KERNEL: bool = false;
pub const ARCH_KERNEL: bool = true;

pub fn device_info() -> (&'static str, &'static str, &'static str, *mut u8, bool, bool) {
    return (BOARD_NAME, BOARD_MANUFACTURER, CPU, UART0, MAIN_KERNEL, ARCH_KERNEL);
}
