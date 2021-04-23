// Kernel defaults
pub static mut IS_OS: bool = false;
pub static mut OS_NAME: &str = "none";
// TODO: Support kernel initramfs like linux
pub static mut IS_INTRAMFS: bool = false;

pub static mut DEVICE_NAME: &str = "default";