// The Novusk kernel panic handler
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! {
    /*extern "C" {
        fn _early_print(args: core::fmt::Arguments);
    }

    unsafe { _early_print(format_args!("{:?}", info)); }*/
    cortex_m_semihosting::hprint!("\nPanic:\n{:?}\n", info);

    loop {  }
}
