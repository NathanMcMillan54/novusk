#![no_std]

pub use novuskinc as novusk;

#[cfg(not(feature = "custom_config"))]
pub mod config;

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub use x86_64::{x86_printk, panic};
    pub use x86_64::kernel as x86_kernel;
    pub use x86_64::include::{asm, other::*};
    pub use x86_64_sound as sound;
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub use arm32;
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
    pub use riscv;
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

    #[cfg(target_arch = "x86_64")]
    pub use libwin;

    #[cfg(target_arch = "x86_64")]
    pub use libost;

    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    // pub use power;
    pub use printk;
}

pub mod drivers {
    #[cfg(target_arch = "x86_64")]
    pub use x86_64::kernel::task as multitask;

    #[cfg(target_arch = "aarch64")]
    pub use rpi;

    pub mod firmware {
        pub use usbd;
    }

    pub mod input {
        #[cfg(target_arch = "x86_64")]
        pub use pc_keyboard;
    }
}

#[cfg(feature = "grub")]
#[no_mangle]
pub extern "C" fn kernel_main() {
    printk::printk!("Kernel main");
}

#[cfg(feature = "grub")]
#[no_mangle]
pub extern "C" fn initramfs_main() {

}