extern "C" { fn rpi_setup(); }

pub unsafe extern "C" fn setup() -> ! {
    rpi_setup();
    loop { asm!("wfe"); }
}