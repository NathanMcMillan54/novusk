#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("bootRV64.S"));

#[cfg(target_arch = "riscv32")]
pub mod bootrv32;

pub mod main;

#[no_mangle]
pub unsafe extern "C" fn die() -> ! {
    panic!("RISCV kernel died, nothing to run");
    loop { asm!("wfi"); }
}
