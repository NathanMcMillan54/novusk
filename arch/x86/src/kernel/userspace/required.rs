pub static mut START_USERSPACE: bool = false;
pub static mut APPLICATION_TYPE: &'static str = "";
pub static mut MAIN_COLOR: &'static str = "";

#[no_mangle]
pub unsafe extern "C" fn set_userspace_info(mut atype: &'static str, mut color: &'static str) {
    APPLICATION_TYPE = atype;
    MAIN_COLOR = color;
}
