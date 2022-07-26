use dif::Dif;

#[no_mangle]
pub static mut DIF: Dif = Dif::new();

pub unsafe fn set_dif() {
    extern "C" {
        static DIF_FILE: &'static [(&'static str, &'static str); 11];
    }

    DIF = DIF.parse(DIF_FILE);
}
