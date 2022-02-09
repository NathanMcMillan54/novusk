#[derive(Debug, Copy, Clone)]
pub struct MailBox {
    pub ptr: *mut u32,
    pub mb_buffer: [u32; 36],
}

impl MailBox {
    pub fn new() -> Self {
        return MailBox {
            ptr: 0x0 as *mut u32,
            mb_buffer: [0; 36],
        };
    }

    pub const fn empty() -> Self {
        return MailBox {
            ptr: 0x0 as *mut u32,
            mb_buffer: [0; 36],
        };
    }
}
