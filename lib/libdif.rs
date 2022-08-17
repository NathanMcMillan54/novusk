use dif::Dif;

extern "C" {
    pub static DIF_FILE: &'static [(&'static str, &'static str)];
}

#[no_mangle]
pub static mut DIF: Dif = Dif::new();

pub unsafe fn set_dif() {
    DIF = DIF.parse(DIF_FILE);
}
