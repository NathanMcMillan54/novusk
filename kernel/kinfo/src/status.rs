use crate::Kinfo;

pub static mut KSTATUS: &'static str = "ok";

#[no_mangle]
pub unsafe extern "C" fn set_status(status: &'static str) {
    KSTATUS = status;
}

impl Kinfo {
    pub fn status_not_ok(&mut self) -> &'static str {
        self.status = "not ok";
        return self.status;
    }
}
