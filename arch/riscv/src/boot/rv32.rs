use core::panic::PanicInfo;
use novuskinc::kernel::start_kernel;

fn boot_setup() {

}

#[entry]
fn rv32_start() -> ! {
    boot_setup();

    unsafe { start_kernel(); }

    panic!("Kernel ended");
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! { loop { } }
