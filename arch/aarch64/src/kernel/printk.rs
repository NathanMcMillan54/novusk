use core::fmt::Arguments;

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn aarch64_printk(args: Arguments) {
    efi_write!("{}", args);
}
