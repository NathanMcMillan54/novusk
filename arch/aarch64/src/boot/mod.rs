use crate::kernel::init::aarch64_kernel_init;
use core::arch::global_asm;
use kinfo::{InfoDisplay, status::KStatus};
use novuskinc::kernel::setup_arch;
use crate::kernel::utils::el;

mod setup;
use self::setup::Aarch64Boot;

#[no_mangle]
pub unsafe extern "C" fn skip_early_boot() {
    #[cfg(any(feature = "qemu_virt"))]
    aarch64_boot_setup();
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    let current_el = el();

    match current_el {
        0 => panic!("Exception level is too low"),
        1 => {},
        _ => panic!("Exception level should be 1, it is {}", current_el),
    }

    let aarch64_boot = Aarch64Boot::new();
    aarch64_boot.setup();

    early_printk!("Setting up kernel...\n");
    setup_arch();

    early_printk!("Starting kernel...\n\n");
    aarch64_kernel_init();

    panic!("Nothing to run");
}

global_asm!(include_str!("boot64.S"));
