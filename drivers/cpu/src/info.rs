pub struct CpuInfo {
    pub architecture: &'static str,
    pub bits: u32,
    pub brand_name: &'static str,
    pub base_address: Option<u32>,
}

impl CpuInfo {
    pub const fn emtpy() -> Self {
        return CpuInfo {
            architecture: "Unknown",
            bits: 0,
            brand_name: "Unknown",
            base_address: None,
        };
    }

    pub fn set(&mut self, arch: &'static str, bits: u32, name: &'static str, address: Option<u32>) {
        self.architecture = arch;
        self.bits = bits;
        self.brand_name = name;
        self.base_address = address;
    }
}
