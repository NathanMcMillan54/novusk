extern "C" {
    fn bios_print(c: char);
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    bios_print('a');

    loop { asm!("hlt"); }
}