use super::{kernel_init};
// use fk_std::libc::c_char;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    // biosPrint(*("Starting kernel...\n".as_ptr() as *const i8));
    kernel_init()
}
