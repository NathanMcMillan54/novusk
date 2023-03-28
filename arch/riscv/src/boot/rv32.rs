use core::panic::PanicInfo;
use riscv::register::mhartid;

#[entry]
fn rv32_start() -> ! {
    let hart_id = mhartid::read();

    panic!("Kernel ended");
}

#[panic_handler]
fn _panic(info: &PanicInfo) -> ! { loop { } }
