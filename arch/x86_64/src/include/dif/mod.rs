use dif::Dif;
use crate::dif::DIF_FILE;

#[no_mangle]
pub static mut DIF: Dif = Dif::empty();

pub unsafe fn init_dif() {
    DIF.set_and_parse(DIF_FILE);
}
