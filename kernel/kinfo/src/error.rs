use crate::KERN_INFO;

#[no_mangle]
pub unsafe extern "C" fn info_error() -> &'static str {
    KERN_INFO = "not ok";
    return KERN_INFO;
}
