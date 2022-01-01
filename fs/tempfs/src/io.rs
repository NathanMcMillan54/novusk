use super::{TempFs, VfsIo};

impl VfsIo for TempFs {
    fn write(name: &str, content: &[u8]) {
        
    }

    fn read(name: &str) -> &[u8] {
        b""
    }
}
