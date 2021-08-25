#![no_std]

#[macro_use] extern crate lazy_static;
use spin::Mutex;

pub mod file;

lazy_static! {
    pub static ref KONFIG: Mutex<Konfig> = Mutex::new(Konfig::new());
}

pub struct Konfig {
    pub config: &'static str,
}

impl Konfig {
    pub fn new() -> Self {
        let mut config_file = file::get_config();
        return Konfig { config: config_file };
    }

    pub fn get(&mut self) -> &str {
        return self.config;
    }
}
