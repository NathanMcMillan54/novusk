use alloc::boxed::Box;
use alloc::string::String;
use super::{dir::Dir, file::File};

impl Dir {
    pub fn new(name: String, path: String) -> Dir {
        Dir {
            dir_name: name,
            dir_path: path
        }
    }
}

impl File {
    pub fn new(name: String, path: String, ftype: String) -> File {
        File {
            file_name: name,
            file_path: path,
            file_type: ftype
        }
    }
}
