use core::fmt::Arguments;

extern "C" {
    fn boot_method() -> &'static str;
}

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn x86_printk(args: Arguments) {
    if boot_method() == "BIOS" {
        // Print from VGA
    } else if boot_method() == "UEFI" {
        efi_write!("{}", args);
    }
}
