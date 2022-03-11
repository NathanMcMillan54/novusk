use core::fmt::{Arguments, Result, Write};
use crate::syscall::syscall;

pub struct HioWriter {
    pub fd: usize,
}

impl HioWriter {
    pub fn new(fd: usize) -> Self {
        return HioWriter {
            fd: fd,
        }
    }

    pub fn write_bytes(&self, bytes: &[u8]) {
        unsafe { syscall(0x05, &[self.fd, bytes.as_ptr() as usize, bytes.len()]); }
    }
}

impl Write for HioWriter {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

pub fn get_hio() -> HioWriter {
    let io_fd = open(b":tt\0", 4);

    if io_fd.is_err() { panic!("Failed to get Semihosting writer"); }

    return HioWriter {
        fd: io_fd.unwrap(),
    };
}

fn open(name: &[u8], mode: usize) -> Result<usize, ()> {
    let ret = unsafe { syscall(0x01, &[name.as_ptr() as usize, mode, name.len() - 1]) };
    return match ret as isize {
        -1 => Err(()),
        fd => Ok(fd as usize),
    }
}
