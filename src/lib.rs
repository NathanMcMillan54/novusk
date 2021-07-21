#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub use x86::{x86_printk, panic};
    pub use x86::kernel as x86_kernel;
    pub use x86::drivers as x86_drivers;
    pub use x86::include::{asm, sys};
}

#[cfg(target_arch = "x86")]
pub mod x86 {
    pub use x86::{x86_printk, panic};
    pub use x86::kernel as x86_kernel;
    pub use x86::drivers as x86_drivers;
    pub use x86::include::{asm, sys};
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub use arm::{arm32_printk, dprint};
    pub use arm::include::asm;
    pub use arm::mm;
}

pub mod aarch64 {
    pub use aarch64::kernel::debug::DebugPrint;
    pub use aarch64::liba64::bmu::Application;
    pub use aarch64::arm::include::asm;
}

pub mod libs {
    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}
