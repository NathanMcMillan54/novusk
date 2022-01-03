use novusk_syscalls::{SysCall, SysCallTable};

pub const WRITE: u32 = 4;
pub const SLEEP: u32 = 24;

extern "C" {
    static mut SYSCALL_TABLE: SysCallTable;
    fn sys_write(byte: u8, arg2: u8, arg3: u8) -> u8;
}

#[cfg(feature = "cortex_m")]
#[no_mangle]
pub unsafe extern "C" fn sys_sleep(sec: u8, arg2: u8, arg3: u8) -> u8 {
    use cortex_m::{delay::Delay, peripheral::Peripherals};

    let peripherals = Peripherals::steal();

    Delay::new(peripherals.SYST, 0).delay_ms(sec as u32 * 1000);

    return arg3;
}

pub unsafe fn syscalls_init() {
    // Novusk syscalls will need to remove alloc in the future
    #[cfg(feature = "cortex_a")]
    return;

    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("ARM32 Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", WRITE, sys_write));

    #[cfg(feature = "cortex_m")]
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_sleep", SLEEP, sys_sleep));
}
