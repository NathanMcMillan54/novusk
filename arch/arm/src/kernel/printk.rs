use core::fmt::Arguments;

#[cfg(target_arch = "arm")]
#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn arm_printk(fmt: Arguments) {
    defmt::debug!("{}", fmt);
}
