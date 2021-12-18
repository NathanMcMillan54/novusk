#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum HexColors {
    Black = 0x000000,
    Red = 0xff0000,
    Blue = 0x0000ff,
    Green = 0x00ff00,
    Yellow = 0xffff00,
    Purple = 0xff00ff,
    Orange = 0xff9900,
    LightGray = 0xa6a6a6,
    White = 0xffffff
}
