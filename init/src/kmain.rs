use super::init::KERNEL;
use super::modules::modules_init;
use super::version::*;
use ps2::mouse::Ps2Mouse;
use kinfo::status::set_status;

pub fn print_version_number() {
    printk!("Running on:");
    printk!("    Novusk v{}.{}.{} {}", MAJOR_VERSION, MINOR_VERSION, REALLY_MINOR_VERSION, VERSION_NAME);
}

unsafe fn reset_gpu_init() {
    KERNEL.lock().kernel_console().uninit();
    KERNEL.lock().gpu_graphics().uninit();
}

fn input_init() {
    let mut mouse = Ps2Mouse::new();
    let info = mouse.init();

    let (succeeded, msg) = info;
    if succeeded {
        printk!("{}", msg);
    } else {
        unsafe { set_status("not ok"); }
        printk!("{}", msg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    KERNEL.lock().gpu_graphics().init();
    KERNEL.lock().kernel_console().init();
    kinfo!("GPU graphics initialized");
    printk!("    Kernel printing re-initialized for higher screen resolution");

    cfg_if! {
        if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
            reset_gpu_init();
            kinfo!("GPU graphics initialized and uninitialized");
        }
    }

    let mut configs = KERNEL.lock().kernel_configs();
    kinfo!("Got kernel configurations");

    if configs.get("KERNEL", "MAJORVERSION").parse::<i32>().unwrap() != MAJOR_VERSION {
        panic!("Kernel config and const versions are different, the config file might be bad and unsafe");
    }

    print_version_number();

    input_init();
    kinfo!("Input devices initialized");

    printk!("\nSetting up main kernel modules...");
    modules_init(configs);
    kinfo!("Initialized main kernel modules");
}
