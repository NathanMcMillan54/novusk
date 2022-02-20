#[path = "mailbox.rs"]
mod mailbox;

use mailbox::{MailBoxResult, MailBoxSender};

pub struct Bcm2837MailBox {
    pub addr: *mut u8,
}

impl Bcm2837MailBox {
    pub fn new(mb_addr: *mut u8) -> Self {
        return Bcm2837MailBox {
            addr: 0x3F00_B880 as *mut u8,
        };
    }
}

impl MailBoxSender for Bcm2837MailBox {

}
