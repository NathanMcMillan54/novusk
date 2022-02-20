use dif::Dif;

// arch/arm/src/include/dif/difs/rpi3b.dif
#[cfg(feature = "rpi3b")]
#[no_mangle]
pub(crate) static mut DIF: Dif = Dif {
    device_name: "RaspberryPi 3b",
    peripheral_addr: Some(0x3F00_0000),
    gpio0_addr: None,
    gpio1_addr: None,
    gpio2_addr: None,
    gpio3_addr: None,
    gpio4_addr: None,
    serial_addr: Some(0x3F20_1000),
    uart_addr: Some(0x3F20_5000),
    fb_addr: None,
    mb_addr: None,
    debug: None
};

