use mailbox::MailBox;

struct Rpi2MbStatus {
    pub bitfield: u32,
}

impl Rpi2MbStatus {
    pub fn new() -> Self {
        return Rpi2MbStatus { bitfield: 0 };
    }

    pub fn read_empty(&self) -> bool {
        return (self.bitfield & (1 << 30)) > 0;
    }

    pub fn write_full(&self) -> bool {
        return (self.bitfield & (1 << 31)) > 0;
    }
}

pub struct Rpi2Mb {
    status: Rpi2MbStatus,
}

impl Rpi2Mb {
    pub fn new() -> Self {
        return Rpi2Mb {
            status: Rpi2MbStatus::new(),
        };
    }
}

impl MailBox for Rpi2Mb {
    fn init(&self) {
        
    }

    fn call(&self, channel: u32) -> Result<(), &str> {

        return Ok(())
    }

    fn clear(&mut self) {

    }
}
