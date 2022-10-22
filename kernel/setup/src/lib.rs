#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

#[path = "../../../lib/libdif.rs"]
mod libdif;

use konfig::KONFIG;

pub mod arch;
pub mod boot;
pub mod kernel;
pub mod types;
pub(crate) mod syscall;
pub(crate) mod user;

pub use boot::BootSetup;
pub use types::SetupReturn;

use types::{str_to_setuptypes, SetupTypes};

pub fn after_kernel_setup() {
    let mut configs = KONFIG.lock();
    let after_str = configs.get("KERNEL", "AFTER");
    let after_setup = str_to_setuptypes(after_str.as_str());

    if after_str == "Nothing" {
        return;
    } else if after_str == "Os" || after_str == "Kernel" || after_str == "Server" || after_str == "BmApp" {
        after_setup.init();
    } else {
        panic!("{} isn't a KERNEL_AFTER option, ending the kernel here (there wasn't much to be done anyway).", after_str);
    }
}
