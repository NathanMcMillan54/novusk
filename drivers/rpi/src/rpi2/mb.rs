use mailbox::MailBox;
use mio::{mmio_read, mmio_write};
use super::registers::*;

#[derive(Copy, Clone)]
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

    pub fn update(&mut self) {
        unsafe { self.bitfield = mmio_read(MAILBOX_STATUS) }
    }
}

pub struct Rpi2Mb {
    bitfield: u32,
    status: Rpi2MbStatus,
}

impl Rpi2Mb {
    pub fn new() -> Self {
        return Rpi2Mb {
            bitfield: 0,
            status: Rpi2MbStatus::new(),
        };
    }

    pub fn get_channel_id(&self) -> usize {
        (self.bitfield & 0x0F) as usize
    }

    pub fn set_channel_id(&mut self, id: u8) {
        self.bitfield = (self.bitfield & 0xFFFFFFF0) | ((id & 0x0F) as u32)
    }

    pub fn get_data(&self) -> usize {
        ((self.bitfield & 0xFFFFFFF0) >> 4) as usize
    }

    pub fn set_data(&mut self, data: u32) {
        self.bitfield = (self.bitfield & 0x0F) | (((data << 4) & 0xFFFFFFF0) as u32)
    }

    pub fn update(&mut self) {
        unsafe { self.bitfield = mmio_read(MAILBOX_READ); }
    }

    pub unsafe fn write(&self) {
        mmio_write(MAILBOX_WRITE, self.bitfield);
    }

    pub fn mailbox_read(&self, channel: usize) -> Self {
        let mut counter: usize = 0;
        let max_count: usize = 20000;
        let mut status = self.status;
        let mut ret= Rpi2Mb::new();

        loop {
            loop {
                status.update();
                counter += 1;
                if counter >= max_count || !status.read_empty() {
                    break;
                }
            }

            ret.update();

            if ret.get_channel_id() == channel || counter >= max_count {
                break;
            }
        }

        return ret;
    }
}

impl MailBox for Rpi2Mb {
    fn init(&self) {
        
    }

    fn call(&mut self, channel: u32) -> Result<(), &str> {
        let mut status = self.status;
        self.set_channel_id(channel as u8);

        loop {
            status.update();
            if !status.write_full() {
                break;
            }
        }

        unsafe { self.write(); }

        return Ok(())
    }

    fn clear(&mut self) {

    }
}
