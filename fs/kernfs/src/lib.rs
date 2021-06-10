#![no_std]

#[macro_use] extern crate alloc;

use libn::libnu::io::fs::{FileAttribute, FileMode, IFile};
use uefi::proto::media::file::File;

pub fn kernfs_init() {
    let mut root = IFile.get();

    let temp = root.open("temp", FileMode::CreateReadWrite, FileAttribute::DIRECTORY);
}
