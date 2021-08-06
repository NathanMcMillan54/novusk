.globl halt
.globl clear_interrupts

halt:
    hlt
    jmp halt

clear_interrupts:
    cli
