use libnu::ktypes::ApplicationType;

pub static mut START_USERSPACE: bool = false;
pub static mut APPLICATION_TYPE: ApplicationType = ApplicationType::None;
pub static mut MAIN_COLOR: &'static str = "";

#[no_mangle]
pub unsafe extern "C" fn set_userspace_info(mut atype: ApplicationType, mut color: &'static str) {
    APPLICATION_TYPE = atype;
    if APPLICATION_TYPE == ApplicationType::None {
        START_USERSPACE = false;
    } else {
        START_USERSPACE = true;
    }
    MAIN_COLOR = color;
}
