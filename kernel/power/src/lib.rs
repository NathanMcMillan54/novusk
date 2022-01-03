#![no_std]

#[macro_use] extern crate printk;

#[cfg(target_arch = "x86_64")]
#[path = "../../../arch/x86_64/src/kernel/power.rs"]
pub(crate) mod arch_power;

#[cfg(target_arch = "xtensa")]
#[path = "../../../arch/xtensa/src/kernel/power.rs"]
pub(crate) mod arch_power;

#[cfg(target_arch = "aarch64")]
#[path = "../../../arch/aarch64/src/kernel/power.rs"]
pub(crate) mod arch_power;

#[cfg(target_arch = "arm")]
#[path = "../../../arch/arm32/src/kernel/power.rs"]
pub(crate) mod arch_power;

#[cfg(target_arch = "riscv32")]
#[path = "../../../arch/riscv/src/kernel/power.rs"]
pub(crate) mod arch_power;

pub mod reboot;

pub struct Power;

impl Power {
    pub fn new() -> Self {
        return Power;
    }
}
