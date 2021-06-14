use core::fmt::Arguments;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn aarch64_printk(args: Arguments) {

}
