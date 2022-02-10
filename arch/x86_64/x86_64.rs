#[allow(warnings)]

#[path = "src/boot/mod.rs"]
pub mod boot;

#[macro_use]
#[path = "src/kernel/mod.rs"]
pub mod kernel;
