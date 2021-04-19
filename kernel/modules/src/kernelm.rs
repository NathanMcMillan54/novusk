extern crate m1;
use m1::{m1_exit, m1_init};

use include::novusk::init;

pub unsafe fn modules_init() {
    init::module_init(m1_init(), m1::MODULE, m1::AUTHOR);
    init::module_exit(m1_exit());
}
