#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate printk;

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

pub mod file;

lazy_static! {
    pub static ref KONFIG: Mutex<Konfig> = Mutex::new(Konfig::new());
}

const NUM_OF_CONFIGS: usize = 5;

pub struct Konfig {
    pub config: &'static str,
}

impl Konfig {
    pub fn new() -> Self {
        let mut config_file = file::get_config();
        return Konfig { config: config_file };
    }

    pub fn get(&mut self, section: &str, config: &str) -> String {
        let formatted_configs: Vec<&str> = self.config.split("\n").collect();
        let mut ret = String::new();

        for i in 0..NUM_OF_CONFIGS {
            if formatted_configs[i].contains(section) && formatted_configs[i].contains(config) {
                ret = formatted_configs[i].replace(section, "").replace(config, "").replace("=", "").replace("_", "");
            }
        }

        return ret;
    }
}
