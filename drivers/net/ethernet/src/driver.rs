use novuskinc::kinfo::{kinfo};

#[derive(Copy, Clone)]
pub struct EthNet {
    pub name: &'static str,
    pub author: &'static str,
    pub connection: bool,
}

impl EthNet {
    pub fn new(driver_name: &'static str, driver_author: &'static str) -> Self {
        return EthNet {
            name: driver_name,
            author: driver_author,
            connection: false
        };
    }

    pub fn driver_info(&mut self) -> (&str, &str) {
        return (self.name, self.author);
    }

    pub fn update_connection(&mut self, updated: bool) {
        self.connection = updated;
    }

    pub fn is_connection(&self) -> bool {
        return self.connection;
    }
}

pub trait EthNetDriver {
    fn write(&mut self, net_text: &[u8]) {

    }

    fn read(&mut self, buf: u8) -> &'static [u8] {
        return b"";
    }

    fn init(&mut self) {

    }
}
