#![no_std]
#![no_main]

mod required;

#[macro_use] extern crate x86_novusk;
use x86_novusk::printk::printk;

use libnu::ktypes::ApplicationType;

pub static mut KERNEL: bool = true;

extern "C" {
    fn set_userspace_info(atype: ApplicationType, color: &'static str);
    fn userspace_init();
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    if !KERNEL {
        printk!("Starting x86_64 OS from kernel extension...");
    } else {
        printk!("x86_64 Kernel extension example");
        KERNEL = false;
        set_userspace_info(ApplicationType::OperatingSystem, "cyan");
        printk!("Starting userspace (again)");
        userspace_init();
    }
    loop {  }
}
