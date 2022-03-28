#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum DisplayMode {
    KernelConsole = 1,
    UserGraphics = 2,
}

impl DisplayMode {
    pub fn as_usize(&self) -> usize {
        return *self as usize;
    }

    pub fn as_u8(&self) -> u8 {
        return *self as usize as u8;
    }
}
