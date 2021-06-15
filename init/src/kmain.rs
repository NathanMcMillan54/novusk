use gpu::init::gpu_init;

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    gpu_init();
    printk!("GPU initialized");
}
