use super::die;

#[no_mangle]
pub unsafe extern "C" fn start_riscv_kernel() -> ! {
    die();
}
