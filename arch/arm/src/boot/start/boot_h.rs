#[no_mangle]
pub unsafe extern "C" fn boot_die() -> ! {
    asm!("wfi");
    boot_die();
}