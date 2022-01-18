#[allow(warnings)]

#[macro_use] extern crate novuskinc;

#[path = "src/boot/mod.rs"]
pub mod boot;

#[path = "src/kernel/mod.rs"]
pub mod kernel;
