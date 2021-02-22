pub mod drivers;
pub mod kernel;

pub fn show_messages() -> bool {
    kernel::SHOW_KERNEL_MESSAGES
}

pub fn machine() -> &'static str {
    drivers::MACHINE
}
