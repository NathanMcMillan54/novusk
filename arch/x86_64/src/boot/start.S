.code64

.globl start_main
.extern main

start_main:
    call clear_regs
    call main

clear_regs:
    mov $0, %ax

    ret
