use drivers::os::setup;
use super::name;
use crate::end_kernel;

pub fn userspace_init() -> ! {
    if name::OSNAME == "none" {
        kprint!("   Nothing to run\n");
        kprint!("   Starting CPU halt...\n");
        unsafe { end_kernel() }
    } else {
        kprint!("   Setting up OS...\n");
        setup::setup();
        kprint!("   Starting OS\n");
        os::main()
    }
}
