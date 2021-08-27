#[derive(Copy, Clone, PartialEq)]
pub enum FileModes {
    Write = 0,
    Read = 1,
    OverWrite = 2,
    Remove = 3,
}

impl FileModes {
    pub fn as_i32(self) -> i32 {
        return self as i32;
    }
}

pub enum DirModes {

}
