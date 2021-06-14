.code32

.extern main
.globl _start

_start:
    cli
    call main
    hlt
