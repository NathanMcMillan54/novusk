#![no_std]

// pub use novuskinc as novusk;

extern crate alloc;
use alloc::boxed::Box;
use lba::Lba;
use printk::printk;
use storage::StorageIo;

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
    pub use aarch64;
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
        pub use unistd;
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub use libwin;

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub use libost;

    pub use libcolor;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}

pub mod drivers {
    #[cfg(target_arch = "x86_64")]
    pub use x86_64::kernel::task as multitask;

    /* #[cfg(target_arch = "aarch64")]
    pub use rpi;*/

    pub mod firmware {
        pub use usbd;
    }

    pub mod gpu {
        pub use gpu::*;

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        pub use armfb;

        #[cfg(target_arch = "x86_64")]
        pub use vgag;

    }

    pub mod input {
        #[cfg(target_arch = "x86_64")]
        pub use pc_keyboard;

        #[cfg(target_arch = "x86_64")]
        pub use kb_mouse;
    }

    pub mod storage {
        pub use storage::*;

        #[cfg(target_arch = "x86_64")]
        pub use lba;
    }
}

#[cfg(feature = "grub")]
#[no_mangle]
pub extern "C" fn kernel_main() {
    printk::printk!("Kernel main\n");
}

#[cfg(feature = "grub")]
#[no_mangle]
pub extern "C" fn initramfs_main() {

}