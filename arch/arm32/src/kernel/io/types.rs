#[derive(Copy, Clone, PartialEq)]
pub enum TextMethods {
    Hio,
    Uart,
    Display,
    Fb,
    None,
}

impl Default for TextMethods {
    fn default() -> Self {
        return TextMethods::None;
    }
}

pub fn str_to_textmethods(string: &str) -> TextMethods {
    return match string {
        "Hio" => TextMethods::Hio,
        "Uart" => TextMethods::Uart,
        "Display" => TextMethods::Display,
        "Fb" => TextMethods::Fb,
        _ => TextMethods::Hio,
    };
}
