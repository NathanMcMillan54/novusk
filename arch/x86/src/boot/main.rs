use super::cpu;

global_asm!(include_str!("header.S"));

extern "C" { fn kernel_main() -> !; }

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        asm!("hlt");
    }

    kernel_main()
}
