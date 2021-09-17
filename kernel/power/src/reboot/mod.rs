pub mod reboot;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/kernel/reboot.rs"]
pub mod x64_power;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/reboot.rs"]
pub mod a64_power;

#[cfg(target_arch = "arm")]
#[path = "../../../../arch/arm/src/cortex_m3/power.rs"]
pub mod a32;
