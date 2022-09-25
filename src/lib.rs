#![no_std]

// pub use novuskinc as novusk;

#[cfg(not(feature = "custom_config"))]
pub mod config;

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub use x86_64::panic;
    pub use x86_64::kernel as x86_kernel;
    pub use x86_64::include::{asm, other::*};
    pub use x86_64_sound as sound;
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub use arm;
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
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub use libwin;

    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub use libost;

    pub use libcolor;
}

pub mod fs {
    pub use vfs;
}

pub mod kernel {
    pub use kinfo;
    pub use printk;
}

pub mod drivers {
    #[cfg(target_arch = "x86_64")]
    pub use x86_64::kernel::task as multitask;

    pub mod firmware {
        pub use usbd;
    }

    pub mod gpu {
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


#[no_mangle]
pub extern "C" fn initramfs_main() {

}

#[no_mangle]
pub extern "C" fn kernel_main() {

}
