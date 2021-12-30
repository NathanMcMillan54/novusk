use super::bootinfo::BootInfo;

#[no_mangle]
pub unsafe extern "C" fn main(bootinfo: &'static BootInfo) -> ! {

    panic!("Kernel should not have reached the end of main()");
}
