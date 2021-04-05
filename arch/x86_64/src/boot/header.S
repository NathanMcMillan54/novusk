.code16

.extern main
.globl _start

_start:
    cld
    cli
    call main

.fill 510-(.-_start), 1, 0
.word 0xaa55
