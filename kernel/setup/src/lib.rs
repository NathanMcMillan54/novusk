#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

#[path = "../../../lib/libdif.rs"]
mod libdif;

pub mod arch;
pub mod boot;
pub mod kernel;
pub mod types;

pub use boot::BootSetup;
pub use types::SetupReturn;
