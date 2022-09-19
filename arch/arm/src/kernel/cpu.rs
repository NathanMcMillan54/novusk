use core::arch::asm;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ArmCpuRegisters {
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub r4: u8,
    pub r5: u8,
    pub r6: u8,
    pub r7: u8,
    pub r8: u8,
    pub r9: u8,
    pub r10: u8,
    pub r11: u8,
    pub r12: u8,
    pub sp: u8,
    pub lr: u8,
    pub pc: u8,
}

impl ArmCpuRegisters {
    pub unsafe fn read() -> Self {
        let mut registers = ArmCpuRegisters {
            ..Default::default()
        };

        asm!("mov {}, r0", out(reg) registers.r0);
        asm!("mov {}, r1", out(reg) registers.r1);
        asm!("mov {}, r2", out(reg) registers.r2);
        asm!("mov {}, r3", out(reg) registers.r3);
        asm!("mov {}, r4", out(reg) registers.r4);
        asm!("mov {}, r5", out(reg) registers.r5);
        asm!("mov {}, r6", out(reg) registers.r6);
        asm!("mov {}, r7", out(reg) registers.r7);
        asm!("mov {}, r8", out(reg) registers.r8);
        asm!("mov {}, r9", out(reg) registers.r9);
        asm!("mov {}, r10", out(reg) registers.r10);
        asm!("mov {}, r11", out(reg) registers.r11);
        asm!("mov {}, r12", out(reg) registers.r12);
        asm!("mov {}, sp", out(reg) registers.sp);
        asm!("mov {}, lr", out(reg) registers.lr);
        asm!("mov {}, pc", out(reg) registers.pc);

        return registers;
    }

    pub unsafe fn write(&self) {
        asm!("mov r0, {}", in(reg) self.r0);
        asm!("mov r1, {}", in(reg) self.r1);
        asm!("mov r2, {}", in(reg) self.r2);
        asm!("mov r3, {}", in(reg) self.r3);
        asm!("mov r4, {}", in(reg) self.r4);
        asm!("mov r5, {}", in(reg) self.r5);
        asm!("mov r6, {}", in(reg) self.r6);
        asm!("mov r7, {}", in(reg) self.r7);
        asm!("mov r8, {}", in(reg) self.r8);
        asm!("mov r9, {}", in(reg) self.r9);
        asm!("mov r10, {}", in(reg) self.r10);
        asm!("mov r11, {}", in(reg) self.r11);
        asm!("mov r12, {}", in(reg) self.r12);
    }
}
