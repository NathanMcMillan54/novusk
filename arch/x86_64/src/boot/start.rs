use bootloader::BootInfo;

#[no_mangle]
pub unsafe extern "C" fn _start(bootinfo: &'static BootInfo) -> ! {


    panic!()
}
