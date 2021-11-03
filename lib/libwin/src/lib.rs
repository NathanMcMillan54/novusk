#![no_std]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This crate is intended for x86_64");

pub(crate) mod display;
pub(crate) mod graphics;

pub struct Window {
    pub title: Option<&'static str>,
    pub size: Option<(u32, u32)>,
    pub color: usize,
    pub open: bool,
}

impl Window {
    pub fn new(win_title: Option<&'static str>, win_size: Option<(u32, u32)>, color: usize) -> Self {
        return Window {
            title: win_title,
            size: win_size,
            color: color,
            open: true,
        };
    }
}
