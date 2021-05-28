use crate::dprint;
use super::info::*;

extern "C" {
    fn kernel_main() -> !;
}

fn print_device_info() {
    dprint!("   Board name: {}\n", BOARD_NAME);
    dprint!("   Board manufacture: {}\n", BOARD_MANUFACTURER);
    dprint!("   CPU: {}\n", CPU);
    dprint!("   Start arch kernel: {}\n", ARCH_KERNEL);
    dprint!("   Start main kernel: {}\n", MAIN_KERNEL);
}

#[no_mangle]
pub unsafe extern "C" fn virt_kernel() -> ! {
    dprint!("Starting Aarch64 Novusk on Qmeu Virt\n");
    dprint!("UART0 initialized\n");
    // Rust can't display u8 or it's just Qemu so it's printed in plain text
    dprint!("   UART0 address: {}\n", "0x0900_000");

    dprint!("Virt Device:\n");
    print_device_info();

    dprint!("Starting bare metal app...\n");
    kernel_main()
}
