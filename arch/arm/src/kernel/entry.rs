use super::cpu::ArmCpuRegisters;
use sys::do_syscall;

#[no_mangle]
pub unsafe extern "C" fn arch_syscall() -> &'static [*const u8] {
    let registers = ArmCpuRegisters::read();

    let sys_num = registers.r3;

    match sys_num as usize {
        _ => {}
    }

    do_syscall(registers.r3 as usize, &[registers.r4 as *const u8, registers.r5 as *const u8, registers.r6 as *const u8, registers.r7 as *const u8]);

    &[]
}
