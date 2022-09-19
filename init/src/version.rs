use novuskinc::version::{MAJOR_VERSION, VERSION_NAME};
use vfs::types::FileModes::ReadOnly;

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
            minor_version: 0,
            really_minor_version: 2,
            version_name: VERSION_NAME,
        };
    }

    pub fn display(&mut self) {
        printk!("________________________\n");
        printk!("| Novusk v{}.{}.{}-{} |\n", self.major_version, self.minor_version, self.really_minor_version, self.version_name);
        printk!("------------------------\n");
    }
}

pub fn novusk_banner() {
    let mut banner = Banner::new();
    banner.display();
}
