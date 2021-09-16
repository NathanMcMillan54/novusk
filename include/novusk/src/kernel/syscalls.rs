extern "C" {
    pub fn sys_write(sys_arg: u8) -> u8;
    pub fn sys_read(sys_arg: u8) -> u8;
    pub fn sys_reboot(sys_arg: u8) -> u8;
    pub fn sys_version(sys_arg: u8) -> u8;
}
