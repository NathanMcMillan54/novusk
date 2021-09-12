.globl halt
.globl clear_interrupts
.globl no_op

halt:
    hlt
    jmp halt

clear_interrupts:
    cli

no_op:
    nop
