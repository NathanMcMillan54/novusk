#![no_std]

pub use novuskinc as novusk;

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub use x86_64::{x86_printk, panic};
    pub use x86_64::kernel as x86_kernel;
    pub use x86_64::include::{asm, other::*};
    pub use x86_64_sound as sound;
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub use arm::include::asm;
    pub use libbmu;
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    pub use aarch64::aarch64_printk;
    pub use aarch64::include::asm;
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

#[cfg(target_arch = "xtensa")]
pub mod xtensa {
    pub use xtensa;
}

pub mod libs {
    pub mod libc {
        pub use memory;
        pub use stdio;
        pub use unistd;
    }

    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use power;
    pub use printk;
}

pub mod drivers {
    #[cfg(target_arch = "x86_64")]
    pub use x86_64::kernel::task as multitask;

    pub use gpu;

    #[cfg(target_arch = "aarch64")]
    pub use rpi;

    pub mod firmware {
        pub use usbd;
    }
}
