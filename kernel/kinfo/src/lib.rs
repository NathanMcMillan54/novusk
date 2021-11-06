#![no_std]

#[macro_use] extern crate printk;

pub mod info;
pub mod macros;
pub mod status;

pub struct Kinfo {
    pub status: &'static str,
    pub msg: Option<&'static str>,
    pub should_panic: bool,
}

impl Kinfo {
    pub fn new() -> Self {
        return Kinfo {
            status: "ok",
            msg: None,
            should_panic: false
        };
    }

    pub fn display_kinfo(&mut self) {
        if self.should_panic {
            panic!("INFO [ {} ] {}", self.status, self.msg.unwrap());
        } else { printk!("INFO [ {} ] {}", self.status, self.msg.unwrap()); }
    }
}
