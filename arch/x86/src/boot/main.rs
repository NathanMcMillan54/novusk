global_asm!(include_str!("header.S"));

extern "C" { fn kernel_main() -> !; }

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    kernel_main()
}