use alloc::boxed::Box;
use alloc::string::String;
use super::{dir::Dir, file::File};

impl Dir {
    pub fn new() -> Dir {
        Dir {
            dir_name: String::new(),
            dir_path: String::new()
        }
    }
}

impl File {
    pub fn new() -> File {
        File {
            file_name: String::new(),
            file_path: String::new(),
            file_type: String::new()
        }
    }
}
