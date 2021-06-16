use init::kmain::kernel_init;

#[no_mangle]
pub unsafe extern "C" fn x86_kernel_init() -> ! {
    // printk!("test");

    kernel_init();
    loop { asm!("hlt"); }
}
