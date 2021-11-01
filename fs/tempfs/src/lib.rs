#![no_std]

pub(crate) use vfs::{Dir, File, NewFs, RootDir, Vfs};

pub mod io;
pub mod root;

pub struct TempFs {
    pub fs: NewFs,
}

impl TempFs {
    pub fn init() -> Self {
        return TempFs {
            fs: NewFs { fs_name: "TempFs" },
        }
    }
}
