cfg_if::cfg_if! {
    if #[cfg(target_arch = "arm")] {
        pub mod alloc;
        pub mod init;
    }
}

pub mod linker_mem;
