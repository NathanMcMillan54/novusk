// If you add a color/shade put it where it would make sense (above or below the color closets to it)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colors {
    White,
    LightGray,
    DarkGray,
    Black,
    Red,
    LightRed,
    Orange,
    Brown,
    Yellow,
    LightGreen,
    Green,
    DarkGreen,
    LightBlue,
    Blue,
    LightCyan,
    Cyan,
    Purple,
    Pink,
}

pub trait ToColors {
    fn as_colors(&self) -> Colors {
        return Colors::Black;
    }
}

impl Colors {
    pub fn color_as_16bit(&self) -> usize {
        return match self {
            Colors::Black => 0,
            Colors::Blue => 1,
            Colors::Green => 2,
            Colors::Cyan => 3,
            Colors::Red => 4,
            Colors::Purple => 5,
            Colors::Brown => 6,
            Colors::LightGray => 7,
            Colors::DarkGray => 8,
            Colors::LightBlue => 9,
            Colors::LightGreen => 10,
            Colors::LightCyan => 11,
            Colors::LightRed => 12,
            Colors::Pink => 13,
            Colors::Yellow => 14,
            Colors::White => 15,
            _ => 0,
        }
    }

    pub fn color_as_hexdec(&self) -> usize {
        return match self {
            Colors::Black => 0x000000,
            Colors::Blue => 0x0000ff,
            Colors::Green => 0x00ff00,
            Colors::Cyan => 0x00a8a8,
            Colors::Red => 0xff0000,
            Colors::Purple => 0xcc00cc,
            Colors::Brown => 0x663300,
            Colors::LightGray => 0xcccccc,
            Colors::DarkGray => 0x333333,
            Colors::LightBlue => 0x9999ff,
            Colors::LightGreen => 0xadebad,
            Colors::LightCyan => 0x00e6e6,
            Colors::LightRed => 0xff6666,
            Colors::Pink => 0xff57ff,
            Colors::Yellow => 0xffff57,
            Colors::White => 0xffffff,
            Colors::Orange => 0xff9933,
            Colors::DarkGreen => 0x009900,

            _ => 0x000000,
        }
    }
}
