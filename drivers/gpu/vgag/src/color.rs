pub use vga::colors::Color16;

pub const BLACK: usize = 0;
pub const BLUE: usize = 1;
pub const GREEN: usize = 2;
pub const CYAN: usize = 3;
pub const RED: usize = 4;
pub const MAGENTA: usize = 5;
pub const BROWN: usize = 6;
pub const LIGHT_GRAY: usize = 7;
pub const DARK_GRAY: usize = 8;
pub const LIGHT_BLUE: usize = 9;
pub const LIGHT_GREEN: usize = 10;
pub const LIGHT_CYAN: usize = 11;
pub const LIGHT_RED: usize = 12;
pub const PINK: usize = 13;
pub const YELLOW: usize = 14;
pub const WHITE: usize = 15;

pub fn convert_usize_to_color16(color: usize) -> Color16 {
    return match color {
        0 => Color16::Black,
        1 => Color16::Blue,
        2 => Color16::Green,
        3 => Color16::Cyan,
        4 => Color16::Red,
        5 => Color16::Magenta,
        6 => Color16::Brown,
        7 => Color16::LightGrey,
        8 => Color16::DarkGrey,
        9 => Color16::LightBlue,
        10 => Color16::LightGreen,
        11 => Color16::LightCyan,
        12 => Color16::LightRed,
        13 => Color16::Pink,
        14 => Color16::Yellow,
        15 => Color16::White,

        _ => Color16::Black,
    };
}