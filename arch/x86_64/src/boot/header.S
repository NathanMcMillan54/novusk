.code32

.extern main

.globl _start
.type _start @function

_start:
    cld
    call main
