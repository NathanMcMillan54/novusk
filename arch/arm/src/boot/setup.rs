use setup::{BootSetup, SetupReturn};

pub struct ArmBoot;

impl ArmBoot {
    pub fn new() -> Self {
        return ArmBoot;
    }

    pub fn setup(&self) {
        let io = self.early_serial_io_init();
        let cpu = self.early_cpu_init();

        if io.0.is_err() {
            panic!("{}", io.1);
        } else if cpu.0.is_err() {
            panic!("{}", cpu.1);
        }
    }
}

impl BootSetup for ArmBoot {
    fn early_serial_io_init(&self) -> SetupReturn {
        (Ok(()), "Early I/O initialized")
    }

    fn disable_wdt(&self) -> SetupReturn {
        (Ok(()), "Watch Dog Timer disabled")
    }

    fn early_cpu_init(&self) -> SetupReturn {
        self.disable_wdt()
    }
}
