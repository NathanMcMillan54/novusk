use core::fmt::Arguments;

#[export_name = "arch_prink"]
#[no_mangle]
pub unsafe extern "C" fn arm32_printk(fmt: Arguments) {

}