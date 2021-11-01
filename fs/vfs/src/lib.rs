#![no_std]

#[macro_use] extern crate alloc;

pub mod dir;
pub mod file;
pub mod root;
pub mod types;
pub mod vfs;

pub use dir::Dir;
pub use file::File;
pub use root::RootDir;
pub use vfs::Vfs;

pub struct NewFs {
    pub fs_name: &'static str,
}
