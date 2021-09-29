#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum FileModes {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    OverWrite,
}

impl FileModes {
    pub fn as_usize(self) -> usize {
        return self as usize;
    }
}
