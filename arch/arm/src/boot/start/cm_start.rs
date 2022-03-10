#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

#[path = "boot_h.rs"]
mod boot_h;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {

    boot_h::boot_die();
}
