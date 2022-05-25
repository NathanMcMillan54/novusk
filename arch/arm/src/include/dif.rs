use dif::Dif;

extern "C" {
    pub static mut DIF_FILE: &'static [(&'static str, &'static str); 11];
}

#[no_mangle]
pub static mut DIF: Dif = Dif::empty();

pub unsafe fn set_dif() {
    DIF.set_and_parse(DIF_FILE);
}
