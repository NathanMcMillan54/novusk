use init::kmain::kernel_init;

#[no_mangle]
pub unsafe extern "C" fn x86_kernel_init() -> ! {
    kernel_init();
    crate::vga_write!("Novusk initialized");
    loop { asm!("hlt"); }
}
