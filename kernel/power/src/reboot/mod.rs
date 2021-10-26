pub mod reboot;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/kernel/reboot.rs"]
pub mod x64_power;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/reboot.rs"]
pub mod a64_power;

#[cfg(target_arch = "arm")]
#[path = "../../../../arch/arm32/src/kernel/power.rs"]
pub mod a32;

#[cfg(target_arch = "riscv32")]
#[path = "../../../../arch/riscv/src/kernel/power/reboot.rs"]
pub mod riscv;
