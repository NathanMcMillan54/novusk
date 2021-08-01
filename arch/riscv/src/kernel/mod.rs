pub mod panic;
pub mod uart;

#[cfg(feature = "hifive")]
#[path = "hifive.rs"]
pub mod board;
