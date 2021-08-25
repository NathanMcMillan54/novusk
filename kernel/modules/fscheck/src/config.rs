use alloc::string::String;
use konfig::KONFIG;

pub fn get_fs() -> String {
    return KONFIG.lock().get("KERNEL", "FS");
}
