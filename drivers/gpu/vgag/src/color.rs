pub use vga::colors::Color16;

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