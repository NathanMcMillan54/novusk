// All these start functions are called from the cortex-m-rt library

extern "C" {
    fn boot_die() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    hprintln!("Starting kernel on Cortex M CPU...");
    boot_die();
}

#[no_mangle]
pub unsafe extern "C" fn __pre_init() {

}
