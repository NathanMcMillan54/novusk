#[derive(Copy, Clone, PartialEq)]
pub enum RpiMb {
    MboxRequest = 0
}

#[derive(Copy, Clone, PartialEq)]
pub enum RpiMbCh {
    MboxChPower = 0,
    MboxChFb = 1,
    MboxChVuart = 2,
    MboxChVChiq = 3,
    MnoxChLeds  = 4,
    MboxChBtns  = 5,
    MboxChTouch = 6,
    MboxChCount = 7,
    MboxChProp  = 8
}

#[derive(Copy, Clone, PartialEq)]
pub enum RpiMboxTag {
    MboxTagSetPower = 0x28001,
    MboxTagSetClkrate = 0x38002,
    MboxTagSetPhywh   = 0x48003,
    MboxTagSetVirtwh  = 0x48004,
    MboxTagSetVirtoff = 0x48009,
    MboxTagSetSetdepth  = 0x48005,
    MboxTagSetPxlordr = 0x48006,
    MboxTagGetFb = 0x40001,
    MboxTagGetPitch = 0x40008,
    MboxTagLast = 0
}

pub struct MailBox {
    pub mailbox: [usize; 36],
}

impl MailBox {
    pub fn new() -> Self {
        return MailBox { mailbox: [0; 36] }
    }

    pub fn clear(&mut self) {
        for i in 0..36 {
            self.mailbox[i] = 0;
        }
    }

    pub fn call(&mut self, ) {

    }
}
