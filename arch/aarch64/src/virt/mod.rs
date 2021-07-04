global_asm!(include_str!("boot.S"));

use crate::kernel::uart0::Uart0;
use core::fmt::Write;

extern "C" {
    fn binit() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn virt_start() -> ! {
    let mut writer = Uart0 {
        address: 0x0900_0000 as *mut u8
    };

    writer.write_fmt(format_args!("{}", "Starting kernel...\n"));
    binit();
}
