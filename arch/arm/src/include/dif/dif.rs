use dif::Dif;

extern "C" {
    pub static mut DIF: Dif;
    pub static DIF_FILE: &'static str;
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub(crate) static DIF: Dif = Dif::empty();
