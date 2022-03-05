/*pub mod init;
pub mod panic;
pub mod power;
pub mod printk;
pub mod uart;*/
pub mod early_printk;
pub mod init;
pub mod panic;
pub mod setup;

#[path = "../../../../kernel/device.rs"]
pub mod device;

#[no_mangle]
pub extern "C" fn test() {}
