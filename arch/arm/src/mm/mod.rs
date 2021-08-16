#[cfg(target_arch = "arm")]
pub mod alloc;

#[cfg(target_arch = "arm")]
pub mod init;

pub mod linker_mem;
