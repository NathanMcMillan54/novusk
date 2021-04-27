global_asm!(include_str!("init.S"));

use core::ptr::write_volatile;

extern "C" { fn kernel_main() -> !; }

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"Novusk Aarch64 Virt board\n";
    for byte in out_str {
        write_volatile(UART0, *byte)
    }
    kernel_main()
}
