#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum FileModes {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    OverWrite,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum FileTypes {
    Text,
    Library,
    Executable,
}

impl FileModes {
    pub fn as_usize(self) -> usize {
        return self as usize;
    }
}

impl FileTypes {
    pub fn as_usize(self) -> usize {
        return self as usize;
    }
}
