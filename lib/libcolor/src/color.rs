#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: u8, background: u8) -> ColorCode {
        ColorCode((background) << 4 | (foreground))
    }
}
