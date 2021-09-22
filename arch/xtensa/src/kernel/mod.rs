pub mod device;
mod panic;
pub mod printk;

#[cfg(feature = "esp32")]
pub(crate) mod esp32;
pub use esp32 as board;
