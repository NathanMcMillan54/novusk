#[path = "../../../../kernel/device.rs"]
pub(crate) mod device;
mod panic;
pub mod printk;
pub mod setup;

pub(crate) static mut DEVICE_NAME: &'static str = "";
