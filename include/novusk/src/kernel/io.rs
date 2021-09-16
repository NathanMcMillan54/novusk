use crate::define_syscall;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/uart.rs"]
pub mod a64_io;

