use dif::{Dif, to_DifFieldNames};

extern "C" {
    pub static DIF_FILE: &'static [(&'static str, &'static str)];
}

pub unsafe fn set_dif() {
    DIF = DIF.parse(DIF_FILE);
}

#[no_mangle]
pub unsafe extern "C" fn get_dif_value(field: &'static str) -> &'static str {
    let filed = to_DifFieldNames(field);

    DIF.get(filed)
}

#[no_mangle]
pub static mut DIF: Dif = Dif::new();

