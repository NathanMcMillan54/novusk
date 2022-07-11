use dif::Dif;

#[no_mangle]
pub(crate) static mut DIF: Dif = Dif::empty();

#[no_mangle]
pub extern "C" fn check_dif_panic() {

}
