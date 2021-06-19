.globl _start
.extern main

_start:
    cli
    call main
