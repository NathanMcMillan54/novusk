#![no_std]

#[macro_use] extern crate lazy_static;
use spin::Mutex;

pub struct CpuInfo {
    pub architecture: &'static str,
    pub brand_name: &'static str,
    pub base_address: Option<u32>,
}

#[no_mangle]
pub static mut CPUINFO: CpuInfo = CpuInfo {
    architecture: "Unknown",
    brand_name: "Unknown",
    base_address: None
};

impl CpuInfo {
    pub fn emtpy() -> Self {
        return CpuInfo {
            architecture: "Unknown",
            brand_name: "Unknown",
            base_address: None,
        };
    }

    pub fn init(&mut self, arch: &'static str, name: &'static str, address: Option<u32>) {
        self.architecture = arch;
        self.brand_name = name;
        self.base_address = address;
    }
}

