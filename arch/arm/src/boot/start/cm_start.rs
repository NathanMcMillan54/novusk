use setup::BootSetup;
use crate::boot::setup::ArmBoot;

extern "C" {
    fn boot_die() -> !;
    static mut __sdata: u64;
    static mut __edata: u64;
    static mut __sidata: u64;
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

    r0::init_data(&mut __sdata, &mut __edata, __sidata as *const u64);

    crate::boot::main::arm_boot_main();

    boot_die();
}
