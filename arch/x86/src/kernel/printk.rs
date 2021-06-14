use core::fmt::Arguments;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn x86_printk(args: Arguments) {
    // Print
}