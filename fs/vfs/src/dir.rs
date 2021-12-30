use alloc::vec::Vec;
use super::file::File;

#[derive(Clone, Debug)]
pub struct Dir {
    pub name: &'static str,
    pub files: Vec<File>,
    pub dirs: Vec<Self>,
}

impl Dir {
    pub fn new(dir_name: &'static str) -> Self {
        return Dir {
            name: dir_name,
            files: vec![],
            dirs: vec![],
        };
    }

    pub fn new_dir(&mut self, dir_name: &'static str) {
        let dir = Dir::new(dir_name);

        self.dirs.push(dir);
    }
}
