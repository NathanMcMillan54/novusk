extern crate ex1;
use ex1::{ex1_end, ex1_init};

use include::novusk::init;

#[no_mangle]
pub unsafe extern "C" fn x86_kernel_modules_init() {
    init::module_init(ex1_init(), ex1::MODULE, ex1::AUTHOR);
    init::module_exit(ex1_end());
}