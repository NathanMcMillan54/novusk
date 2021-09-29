use mio::{mmio_read, mmio_write};

pub struct Vfs {
    pub name: &'static str,
    pub author: &'static str,
}

impl Vfs {
    pub fn new(vfs: Vfs) -> Self {
        return vfs;
    }

    pub fn info(&mut self) -> (&str, &str) {
        return (self.name, self.author);
    }

    pub fn open(&mut self, file: &str, mode: usize) {

    }

    pub fn read(&mut self, buf: u8) -> &[u8] {
        return b"";
    }
}
