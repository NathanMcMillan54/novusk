#![no_std]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This crate is intended for x86_64");

pub struct Window {
    pub title: Option<&'static str>,
    pub size: Option<(u32, u32)>,
    pub open: bool,
}

impl Window {
    pub fn new(win_title: Option<&'static str>, win_size: Option<(u32, u32)>) -> Self {
        return Window {
            title: win_title,
            size: win_size,
            open: true,
        };
    }
}
