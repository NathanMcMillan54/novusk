#[derive(Copy, Clone, PartialEq)]
pub enum SerialMethods {
    Hio,
    Uart,
    Display,
    Fb,
    None,
}

impl Default for SerialMethods {
    fn default() -> Self {
        return SerialMethods::None;
    }
}

pub fn str_to_serialmethods(string: &str) -> SerialMethods {
    return match string {
        "Hio" => SerialMethods::Hio,
        "Uart" => SerialMethods::Uart,
        "Display" => SerialMethods::Display,
        "Fb" => SerialMethods::Fb,
        _ => SerialMethods::Hio,
    };
}
