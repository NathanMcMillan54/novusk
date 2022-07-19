use core::arch::global_asm;

pub mod init;
pub mod irq;
pub mod kernel;
pub mod printk;
pub mod setup;
pub mod syscall;

#[path = "../../../../drivers/drivers.rs"]
pub(crate) mod drivers;

#[path = "../../../../kernel/panic.rs"]
pub mod panic;

#[cfg(feature = "cortex_m")]
global_asm!(include_str!("cm_entry.S"));
