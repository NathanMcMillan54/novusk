pub mod cpu;
pub mod early_printk;
pub mod init;
pub mod panic;
pub mod setup;

#[path = "../../../../kernel/device.rs"]
pub mod device;

#[path = "../../../../drivers/drivers.rs"]
pub mod drivers;

#[no_mangle]
pub extern "C" fn test() {}
