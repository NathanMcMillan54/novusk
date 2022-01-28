use crate::include::dif::dif_init;

pub unsafe fn arm_kernel_init() {
    dif_init();
    crate::early_printk!("\ndif init\n");
}
