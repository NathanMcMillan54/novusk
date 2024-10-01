use core::arch::asm;

#[derive(Copy, Clone, Debug)]
pub struct ArchCpuRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl ArchCpuRegisters {
    pub fn get() -> Self {
        let mut rax = 0;
        let mut rbx = 0;
        let mut rcx = 0;
        let mut rdx = 0;
        let mut rbp = 0;
        let mut rsp = 0;
        let mut rsi = 0;
        let mut rdi = 0;
        let mut r8 = 0;
        let mut r9 = 0;
        let mut r10 = 0;
        let mut r11 = 0;
        let mut r12 = 0;
        let mut r13 = 0;
        let mut r14 = 0;
        let mut r15 = 0;

        unsafe {
            asm!("mov {},rax", out(reg) rax);
            asm!("mov {},rbx", out(reg) rbx);
            asm!("mov {},rcx", out(reg) rcx);
            asm!("mov {},rdx", out(reg) rdx);
            asm!("mov {},rbp", out(reg) rbp);
            asm!("mov {},rsp", out(reg) rsp);
            asm!("mov {},rsi", out(reg) rsi);
            asm!("mov {},rdi", out(reg) rdi);
            asm!("mov {},r8", out(reg) r8);
            asm!("mov {},r9", out(reg) r9);
            asm!("mov {},r10", out(reg) r10);
            asm!("mov {},r11", out(reg) r11);
            asm!("mov {},r12", out(reg) r12);
            asm!("mov {},r13", out(reg) r13);
            asm!("mov {},r14", out(reg) r14);
            asm!("mov {},r15", out(reg) r15);
        }
        
        return ArchCpuRegisters {
            rax,
            rbx,
            rcx,
            rdx,
            rbp,
            rsp,
            rsi,
            rdi,
            r8,
            r9,
            r10,
            r11,
            r12,
            r13,
            r14,
            r15,
        };
    }

    pub fn write(&self) {
        unsafe {
            asm!("mov rax, {}", in(reg) self.rax);
            asm!("mov rbx, {}", in(reg) self.rbx);
            asm!("mov rcx, {}", in(reg) self.rcx);
            asm!("mov rdx, {}", in(reg) self.rdx);
            asm!("mov rbp, {}", in(reg) self.rbp);
            asm!("mov rsp, {}", in(reg) self.rsp);
            asm!("mov rsi, {}", in(reg) self.rsi);
            asm!("mov rdi, {}", in(reg) self.rdi);
            asm!("mov r8, {}", in(reg) self.r8);
            asm!("mov r9, {}", in(reg) self.r9);
            asm!("mov r10, {}", in(reg) self.r10);
            asm!("mov r11, {}", in(reg) self.r11);
            asm!("mov r12, {}", in(reg) self.r12);
            asm!("mov r13, {}", in(reg) self.r13);
            asm!("mov r14, {}", in(reg) self.r14);
            asm!("mov r15, {}", in(reg) self.r15);
        }
    }
}
