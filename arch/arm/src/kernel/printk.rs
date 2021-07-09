use core::fmt::Arguments;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn arm_printk(fmt: Arguments) {
    defmt::write!("{}", fmt);
}
