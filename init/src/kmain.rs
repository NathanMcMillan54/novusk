use gpu::gpu_graphics_init;
use super::version::*;

pub fn print_version_number() {
    printk!("Running on:");
    printk!("   Novusk v{}.{}.{} {}", MAJOR_VERSION, MINOR_VERSION, REALLY_MINOR_VERSION, VERSION_NAME);
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    gpu_graphics_init();
    kinfo!("GPU graphics initialized");

    print_version_number();
}
