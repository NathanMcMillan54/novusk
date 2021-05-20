pub mod uefi_rpi3;

pub struct Device {
    pub board: &'static str,
    pub manufacture: &'static str,
    pub cpu: &'static str,
    pub uart0: *mut u8,
    pub main_kernel: bool,
    pub arch_kernel: bool,
}

pub static mut DEVICE_INFO: Device = Device {
    board: "",
    manufacture: "",
    cpu: "",
    uart0: 0 as *mut u8,
    main_kernel: false,
    arch_kernel: false
};
