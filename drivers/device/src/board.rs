pub struct Device {
    pub name: &'static str,
    pub peripheral_addr: *mut u8,
    pub uart_addr: *mut u8,
    pub fb_addr: *mut u8,
    pub arch_init: bool,
    pub kernel_init: bool,
    pub device_specific_kernel: Option<unsafe extern "C" fn()>,
}

impl Device {
    pub const fn empty() -> Self {
        return Device {
            name: "Unknown",
            peripheral_addr: 0x0 as *mut u8,
            uart_addr: 0x0 as *mut u8,
            fb_addr: 0x0 as *mut u8,
            arch_init: true,
            kernel_init: true,
            device_specific_kernel: None
        }
    }

    pub fn set(&mut self, dev: Device) {
        self.name = dev.name;
        self.peripheral_addr = dev.peripheral_addr;
        self.uart_addr = dev.uart_addr;
        self.fb_addr = dev.fb_addr;
        self.arch_init = dev.arch_init;
        self.kernel_init = dev.kernel_init;
        self.device_specific_kernel = dev.device_specific_kernel;
    }

    pub fn run_board_specific_kernel(&self) {
        if self.device_specific_kernel.is_none() {
            panic!("Can't find device specific kernel for {}", self.name);
        } else {
            unsafe { self.device_specific_kernel.unwrap()(); }
        }
    }
}
