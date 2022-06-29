use crate::include::dif::DIF;
use novuskinc::platform::early_device_init;
use novuskinc::serial::early_serial_init;
use setup::{BootSetup, SetupReturn};

pub struct ArmBoot;

impl ArmBoot {
    pub fn new() -> Self {
        return ArmBoot;
    }

    pub fn setup(&self) {
        let early_dev = self.early_device_init();
        let io = self.early_serial_io_init();
        let cpu = self.early_cpu_init();

        if early_dev.0.is_err() {
            panic!("{}", early_dev.1);
        } else if io.0.is_err() {
            panic!("{}", io.1);
        } else if cpu.0.is_err() {
            panic!("{}", cpu.1);
        }
    }
}

impl BootSetup for ArmBoot {
    fn early_device_init(&self) -> SetupReturn {
        unsafe {
            if early_device_init() == 0 {
                (Ok(()), "Device initialized")
            } else { (Err("Device init error"), "Failed to initialize device") }
        }
    }

    fn early_serial_io_init(&self) -> SetupReturn {
        unsafe {
            if DIF.get("EnableSerial").1.parse::<bool>().unwrap() == false {
                return (Ok(()), "Serial doesn't need to be initialized");
            }
        }

        unsafe {
            if early_serial_init() == 0 {
                return (Ok(()), "Early I/O initialized");
            } else { return (Err("Serial init error"), "Failed to initialize early serial I/O"); }
        }
    }

    fn disable_wdt(&self) -> SetupReturn {
        (Ok(()), "Watch Dog Timer disabled")
    }

    fn early_cpu_init(&self) -> SetupReturn {
        self.disable_wdt()
    }
}
