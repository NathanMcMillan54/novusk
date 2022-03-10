use setup::BootSetup;
use crate::boot::setup::ArmBoot;

extern "C" {
    fn boot_die() -> !;
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let boot = ArmBoot::new();

    if boot.linker_setup().0.is_err() {
        panic!("Failed to clear BSS");
    }

    boot_die();
}
