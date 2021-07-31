use super::die;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    die();
}
