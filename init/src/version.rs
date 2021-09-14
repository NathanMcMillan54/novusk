use novuskinc::version::*;

struct Banner {
    pub kernel: &'static str,
    pub major_version: i32,
    pub minor_version: i32,
    pub really_minor_version: i32,
    pub version_name: &'static str,
}

impl Banner {
    pub fn new() -> Self {
        return Banner {
            kernel: "Novusk",
            major_version: MAJOR_VERSION,
            minor_version: MINOR_VERSION,
            really_minor_version: REALLY_MINOR_VERSION,
            version_name: VERSION_NAME,
        };
    }

    pub fn display(&mut self) {
        printk!("______________________");
        printk!("| Novusk v{}.{}.{}-{} |", MAJOR_VERSION, MINOR_VERSION, REALLY_MINOR_VERSION, VERSION_NAME);
        printk!("----------------------");
    }
}

pub fn novusk_banner() {
    let mut banner = Banner::new();
    banner.display();
}
