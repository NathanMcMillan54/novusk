pub static mut KSTATUS: &'static str = "ok";

#[no_mangle]
pub unsafe extern "C" fn set_status(status: &'static str) {
    KSTATUS = status;
}
