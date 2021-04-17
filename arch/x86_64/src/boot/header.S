.code32

.globl _start
.type _start @function

.extern stack_top
.extern main

_start:
    movl $stack_top, %esp

    call main
    hlt
