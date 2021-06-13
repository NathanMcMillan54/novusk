.code32

.globl _start
.extern main

_start:
    call main
    hlt
