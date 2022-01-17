pub unsafe fn die() -> ! {
    panic!("Kernel died");
}

#[no_mangle]
pub unsafe extern "C" fn boot_init() {

}
