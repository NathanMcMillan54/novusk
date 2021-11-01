use alloc::vec::Vec;
use super::file::File;

#[derive(Clone, Debug)]
pub struct Dir {
    pub name: &'static str,
    pub files: Option<Vec<File>>,
    pub dirs: Option<Vec<Self>>,
}

impl Dir {
    pub fn new(dir_name: &'static str) -> Self {
        return Dir {
            name: dir_name,
            files: None,
            dirs: None,
        };
    }

    pub fn new_dir(&mut self, dir_name: &'static str) {
        let dir = Dir::new(dir_name);

        self.dirs.clone().unwrap().push(dir);
    }
}
