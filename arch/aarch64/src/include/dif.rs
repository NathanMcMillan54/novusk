use dif::Dif;

#[no_mangle]
pub(crate) static mut DIF: Dif = Dif::empty();
