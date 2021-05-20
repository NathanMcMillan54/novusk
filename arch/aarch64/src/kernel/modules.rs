// Modules
use armm::{armm_end, armm_init};

use novusk::module::{module_end, module_init};

pub unsafe fn modules_init() {
    module_init(armm_init(), "Nathan McMillan <nathanmcmillan54@gmail.com", "armm");
    module_end(armm_end());
}
