use super::syscall::check_sys_nums;

pub fn user_setup() {
    check_sys_nums();
}
