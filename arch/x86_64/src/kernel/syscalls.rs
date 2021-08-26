#[no_mangle]
pub extern "C" fn sys_write(write: u8) {
    printk!("{}", write);
}
