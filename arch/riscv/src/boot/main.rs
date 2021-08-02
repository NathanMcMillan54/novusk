use super::die;
use crate::kernel::uart::Uart;

unsafe fn rv64_board_init() {
    Uart::new(0x1000_0000).uart_init();
}

#[no_mangle]
pub unsafe extern "C" fn start_riscv_kernel() -> ! {
    #[cfg(target_arch = "riscv64")]
    rv64_board_init();

    die();
}
