use core::arch::global_asm;
use core::ptr::write_volatile;

mod setup;
use self::setup::Aarch64Boot;

#[no_mangle]
pub unsafe extern "C" fn skip_early_boot() {
    #[cfg(any(feature = "qemu_virt"))]
    aarch64_boot_setup();
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    write_volatile(0x3F00_0000 as *mut u8, b'A');
    /*let current_el = el();

    match current_el {
        0 => panic!("Exception level is too low"),
        1 => {},
        _ => panic!("Exception level should be 1, it is {}", current_el),
    }*/
    panic!("Nothing to run");
}

#[no_mangle]
pub extern "C" fn irq_handler() {

}

#[no_mangle]
pub extern "C" fn DefaultHandler() {}

global_asm!(include_str!("boot64.S"));
