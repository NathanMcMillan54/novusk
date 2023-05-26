#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color4Bit {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color8Bit(u8);

impl Color8Bit {
    pub fn value(self) -> u8 {
        self.0
    }

    #[allow(non_snake_case)]
    pub fn from_Color4Bit(color: Color4Bit) -> Self {
        Color8Bit(color as u8)
    }
}
