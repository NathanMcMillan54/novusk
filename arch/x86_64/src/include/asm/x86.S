.globl halt
.globl clear_interrupts
.globl no_op
.globl enable_interrupts

halt:
    hlt
    jmp halt

clear_interrupts:
    cli

no_op:
    nop

enable_interrupts:
    sti
