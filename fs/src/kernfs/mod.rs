pub mod dir;
pub mod file;
pub mod implfs;

use dir::Dir;
use alloc::string::{ToString, String};

pub unsafe fn kernelfs_init() {
    // let root = Dir::new("root".to_string(), "/".to_string());
    // printk!("   Created root directory");
}
