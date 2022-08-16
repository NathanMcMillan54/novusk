pub mod irq;
pub mod panic;
pub mod power;
pub mod printk;
pub mod platform;
pub mod setup;

#[path = "../../../../drivers/empty.rs"]
pub(crate) mod empty;
