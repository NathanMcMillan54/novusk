use core::fmt::Debug;

#[cfg(target_arch = "x86_64")]
pub fn convert_to_vga_color(color: usize) -> vgag::Color16 {
    use vgag::Color16;

    match color {
        0 => return Color16::Black,
        1 => return Color16::Black,
        2 => return Color16::Green,
        3 => return Color16::Cyan,
        4 => return Color16::Red,
        5 => return Color16::Magenta,
        6 => return Color16::Brown,
        7 => return Color16::LightGrey,
        8 => return Color16::DarkGrey,
        9 => return Color16::LightBlue,
        10 => return Color16::LightGreen,
        11 => return Color16::LightCyan,
        12 => return Color16::LightRed,
        13 => return Color16::Pink,
        14 => return Color16::Yellow,
        15 => return Color16::White,

        _ => return Color16::Black
    }
}