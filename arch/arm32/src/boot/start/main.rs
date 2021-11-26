use crate::boot::cpu::early_cpu_init;

#[no_mangle]
pub extern "C" fn bmain() -> ! {
    early_cpu_init();

    loop {  }
}
