pub type SocAddr = (&'static str, u32);


#[derive(PartialEq)]
pub struct SocInfo {
    pub known: bool,
    pub name: &'static str,
    pub addresses: [SocAddr; 20],
}

impl SocInfo {
    pub fn get(&self, name: &'static str) -> Option<u32> {
        for n in 0..self.addresses.len() {
            if name == self.addresses[n].0 {
                return Some(self.addresses[n].1);
            }
        }

        return None;
    }
}

impl Default for SocInfo {
    fn default() -> Self {
        return SocInfo {
            known: false,
            name: "Unknown - not set",
            addresses: [("None", 0x0); 20],
        };
    }
}

