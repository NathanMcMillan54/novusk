pub struct CpuInfo {
    pub architecture: &'static str,
    pub brand_name: &'static str,
    pub base_address: Option<u32>,
}

impl CpuInfo {
    pub const fn emtpy() -> Self {
        return CpuInfo {
            architecture: "Unknown",
            brand_name: "Unknown",
            base_address: None,
        };
    }

    pub fn set(&mut self, arch: &'static str, name: &'static str, address: Option<u32>) {
        self.architecture = arch;
        self.brand_name = name;
        self.base_address = address;
    }
}
