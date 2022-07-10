use dif::Dif;

extern "C" {
    pub static mut DIF_FILE: &'static [(&'static str, &'static str); 11];
}

#[no_mangle]
pub static mut DIF: Dif = Dif::empty();

pub unsafe fn set_dif() {
    DIF.set_and_parse(DIF_FILE);
}

#[no_mangle]
pub extern "C" fn check_dif_panic() {
    let shutdown_line = unsafe { DIF }.get("ShutdownOnPanic");

    if shutdown_line.1.parse::<bool>().unwrap_or(false) {
        // set_power_mode(PM_SHUTDOWN);
    }
}
