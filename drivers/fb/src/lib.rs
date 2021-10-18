#![no_std]

pub struct Fb {
    pub name: &'static str,
    pub address: *mut u64,
}

impl Fb {
    pub fn new(fb_name: &str, fb_address: usize) -> Self {
        return Fb {
            name: fb_name,
            address: fb_address,
        };
    }

    pub unsafe fn fb_write(&mut self, write: u8) {
        *self.address = write;
    }
}
