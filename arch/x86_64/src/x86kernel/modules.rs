use include::novusk::init::{module_exit, module_init};

use ex1::{ex1_exit, ex1_init};

pub unsafe fn x86_64_kernel_modules_init() {
    module_init(ex1_init(), "ex1", "Nathan McMillan");
    module_exit(ex1_exit());
}
