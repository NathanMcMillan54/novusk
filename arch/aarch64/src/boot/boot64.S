// Boots on Cortex-a57
.section ".text.boot"

.extern aarch64_boot_setup
.extern skip_early_boot

.globl _start

_start:
    mrs x19, mpidr_el1
    and x19, x19, #3
    cbnz x19, boot_loop

    ldr x1, =_start
    mov sp, x1

    bl skip_early_boot

    // Early boot setup
    bl el_setup

    // From ``mod.rs``
    bl aarch64_boot_setup

    b boot_loop

el_setup:
    msr SPsel, #1

    mrs x0, CurrentEL
    and x0, x0, #0b1100
    lsr x0, x0, #2

switch_to_el2:
    // Switch to EL1 if CPU is already in EL2
    cmp x0, #2
    beq switch_to_el1

    // Switch to EL2
    mov x0, #0x5b1
    msr SCR_EL3, x0

    mov x0, #0x3c9
    msr SPSR_EL3, x0

    // Now switch to EL2
    adr x0, switch_to_el1
    msr elr_el3, x0
    eret

switch_to_el1:
    cmp x0, #1
    beq el_setup_end

    // Switches to EL2
    adrp x0, _start
    sub x0, x0, x19, lsl #16
    msr sp_el1, x0

    mov x0, #0x0002
    movk x0, #0x8000, lsl #16
    msr hcr_el2, x0

    msr cptr_el2, xzr

    mrs x0, cpacr_el1
    orr x0, x0, #(0x3 << 20)
    msr cpacr_el1, x0

    mov x0, #0x0800
    movk x0, #0x30d0, lsl #16
    msr sctlr_el1, x0

    mov x0, #0x3c5
    msr spsr_el2, x0

    mrs x0, cnthctl_el2
    orr x0, x0, #3
    msr cnthctl_el2, x0
    msr cntvoff_el2, xzr

    // Switch to EL1
    msr elr_el2, lr
    eret

el_setup_end:
    adrp x0, _start
    sub x0, x0, x19, lsl #16
    mov sp, x0
    ret

start_kernel:
    ret

boot_loop:
    wfe
    b .
