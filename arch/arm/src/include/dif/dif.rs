use dif::Dif;

extern "C" {
    #[cfg(target_arch = "aarch64")]
    pub static mut DIF: Dif;
    pub static DIF_FILE: &'static str;
}

#[cfg(target_arch = "arm")]
#[no_mangle]
pub(crate) static mut DIF: Dif = Dif::empty();
