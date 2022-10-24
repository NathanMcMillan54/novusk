pub struct SocInfo {
    pub name: &'static str,
    pub addresses: &'static [(&'static str, u32); 10],
}

impl SocInfo {
    pub fn get(&self, name: &'static str) -> u32 {
        for addr in 0..self.addresses.len() {
            if self.addresses[addr].0 == name {
                return self.addresses[addr].1;
            }
        }

        return 0;
    }
}
