use nkuefi::st::st;
use uefi::proto::console::gop::GraphicsOutput;

pub unsafe extern "C" fn gop_reinit() -> &'static mut GraphicsOutput<'static> {
    let bs = st().as_ref().boot_services();
    if let Ok(gop) = bs.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = &mut *gop.get();
        printk!("GOP initialized");
        return gop;
    } else {
        printk!("Failed to initialize GOP");
        gop_reinit()
    }
}
