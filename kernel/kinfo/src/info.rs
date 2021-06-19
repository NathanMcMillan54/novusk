pub static mut KINFO: &'static str = "ok";

#[no_mangle]
pub unsafe extern "C" fn set_info(info: &'static str) {
    KINFO = info;
}
