/*pub mod init;
pub mod panic;
pub mod power;
pub mod printk;
pub mod uart;*/
pub mod panic;
pub mod setup;

#[path = "../../../../kernel/device.rs"]
pub mod device;
