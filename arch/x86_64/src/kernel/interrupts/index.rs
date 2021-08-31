use super::{PIC_1_OFFSET, PIC_2_OFFSET, PIC_12_OFFSET};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Mouse = PIC_12_OFFSET,
}
