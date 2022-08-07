use rpi::Rpi2;

#[no_mangle]
pub extern "C" fn device_init() -> (Result<(), &'static str>, &'static str) {
    let mut pi = Rpi2::new();
    pi.init();

    return (Ok(()), "RPi 2");
}
