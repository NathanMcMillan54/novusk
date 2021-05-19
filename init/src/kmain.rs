use novusk::module::{module_end, module_init};

// Modules
use m1::{m1_exit, m1_init};

#[no_mangle]
pub unsafe extern "C" fn modules_init() {
    module_init(m1_init(), "Nathan McMillan <nathanmcmillan54@gmail.com>", "m1");
    kinfo!("M1 Module initialized");
    module_end(m1_exit());
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    modules_init();
    kinfo!("Kernel modules initialized");

}

