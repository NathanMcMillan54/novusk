use super::{TempFs, VfsIo};

impl VfsIo for TempFs {
    fn write(&mut self, name: &str, content: &[u8]) {
        
    }

    fn read(&mut self, name: &str) -> &[u8] {
        b""
    }
}
