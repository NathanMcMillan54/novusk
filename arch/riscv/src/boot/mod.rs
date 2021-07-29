pub mod main;

#[no_mangle]
pub unsafe extern "C" fn die() -> ! {
    panic!("RISCV kernel died, nothing to run");
    loop { asm!("wfi"); }
}
