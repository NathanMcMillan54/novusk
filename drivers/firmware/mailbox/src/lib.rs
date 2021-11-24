#![no_std]

pub trait MailBox {
    const REQUEST: u32 = 0;

    fn init(&self) {

    }

    fn call(&self, channel: u32) -> Result<(), &str> {
        return Ok(())
    }

    fn clear(&mut self) {

    }
}
