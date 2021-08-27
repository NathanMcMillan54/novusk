use crate::modes::FileModes;

pub struct File {
    pub name: &'static str,
    pub mode: i32,
}

impl File {
    pub fn open(file_name: &'static str, file_mode: FileModes) -> Self {
        return File { name: file_name, mode: file_mode.as_i32() }
    }

    pub fn write(&mut self, buf: &[u8], write: &str) {

    }
}
