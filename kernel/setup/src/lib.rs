#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate printk;


pub mod arch;
pub mod boot;
pub mod kernel;

pub use boot::BootSetup;
pub use kernel::ArchKernelSetup;

pub type SetupReturn = (Result<(), &'static str>, &'static str);
