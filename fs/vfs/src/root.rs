use crate::{Dir, NewFs};

#[derive(Clone, Debug)]
pub struct RootDir {
    pub root: Dir,
}

impl RootDir {
    pub fn make() -> Self {
        return RootDir {
            root: Dir::new("Root"),
        };
    }
}
