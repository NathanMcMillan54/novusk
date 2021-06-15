use uefi::proto::console::gop::GraphicsOutput;
use nkuefi::st::st;

pub unsafe fn gop_init() {
    let bs = st().as_ref().boot_services();
    if let Ok(gop) = bs.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = &mut *gop.get();
        printk!("GOP initialized");
    } else {
        printk!("Failed to initialize GOP");
    }
}
