#![no_std]

extern "C" {
    pub fn outb(port: u32, b: u32);
    pub fn inb(port: u32) -> u32;

    pub fn disable_irqs();
    pub fn enable_irqs();
    pub fn irqs_enabled() -> u8;
}


pub fn arch_cpu_registers_write(registers: arch::regs::ArchCpuRegisters) {
    registers.write();
}

pub fn arch_cpu_registers_read() -> arch::regs::ArchCpuRegisters {
    arch::regs::ArchCpuRegisters::get()
}

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
pub mod arch;