use drivers::os::setup;
use super::name;
use super::super::end_kernel;

pub fn userspace_init() -> ! {
    if name::OSNAME == "none" {
        kprint!("   Nothing to run\n");
        unsafe { end_kernel() }
    } else {
        kprint!("   Setting up OS...\n");
        setup::setup();
        kprint!("   Starting OS\n");
        os::main()
    }
}
