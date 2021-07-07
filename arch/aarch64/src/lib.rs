#![no_std]
#![feature(asm, global_asm, llvm_asm)]
#![feature(core_intrinsics)]

pub mod boot;
pub mod kernel;

#[cfg(feature = "qemu_virt")]
pub mod virt;
