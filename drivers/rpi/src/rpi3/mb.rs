use mailbox::MailBox;

pub struct Rpi3Mb {
    pub mb_buffer: [u32; 36],
}

impl Rpi3Mb {
    pub fn new() -> Self {
        return Rpi3Mb { mb_buffer: [0; 36] };
    }
}

impl MailBox for Rpi3Mb {
    const REQUEST: u32 = 0;

    fn init(&self) {

    }

    fn call(&self, channel: u32) -> Result<(), &str> {
        return Ok(())
    }

    fn clear(&mut self) {
        for i in 0..36 {
            self.mb_buffer[i] = 0;
        }
    }
}