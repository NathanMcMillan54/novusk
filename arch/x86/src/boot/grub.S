.code32

.globl _grub_start
.extern main

_grub_start:
    call main
