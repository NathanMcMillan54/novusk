use dif::Dif;

extern "C" {
    #[cfg(target_arch = "aarch64")]
    pub static mut DIF: Dif;
    pub static DIF_FILE: &'static [&'static str; 11];
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub static mut DIF: Dif = Dif {
    device_name: "",
    peripheral_addr: None,
    gpio0_addr: None,
    gpio1_addr: None,
    gpio2_addr: None,
    gpio3_addr: None,
    gpio4_addr: None,
    serial_addr: None,
    uart_addr: None,
    fb_addr: None,
    mb_addr: None,
    debug: None
};
