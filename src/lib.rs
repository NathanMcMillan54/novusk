#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub use x86::{x86_printk, panic};
    pub use x86::kernel as x86_kernel;
    pub use x86::drivers as x86_drivers;
    pub use x86::include::{asm, other::*, sys};
    pub use x86_64_sound as sound;
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
    pub use libbmu;
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    pub use aarch64::arm::include::asm;
    pub use libbmu;
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod riscv {
    pub use riscv::riscv_printk;
    pub use riscv::include::asm;
    pub use riscv::kernel::device::DEVICE;
    pub use riscv::kernel::panic::panic;
    pub use libbmu;
}

pub mod libs {
    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}
