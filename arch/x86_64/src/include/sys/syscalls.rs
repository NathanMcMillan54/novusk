#[no_mangle]
pub unsafe extern "C" fn sys_write(bytes: &[u8]) {
    use crate::kernel::io::_x86_printk;
    use core::str::from_utf8_unchecked;
    _x86_printk(format_args!("{}", from_utf8_unchecked(bytes)));
}

#[no_mangle]
pub unsafe extern "C" fn sys_read(bytes: &[u8]) {
    use crate::kernel::io::ps2_keyboard_input;
    ps2_keyboard_input();
}
