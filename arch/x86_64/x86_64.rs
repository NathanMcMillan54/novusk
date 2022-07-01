#[allow(warnings)]

#[path = "src/boot/mod.rs"]
pub mod boot;

#[path = "src/dif.rs"]
pub mod dif;

#[path = "src/include/mod.rs"]
pub mod include;

#[macro_use]
#[path = "src/kernel/mod.rs"]
pub mod kernel;

#[path = "src/mm/mod.rs"]
pub mod mm;
