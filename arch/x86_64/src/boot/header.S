.code32
.globl _start
.extern main

_start:
    call bootloader_start_novusk
    hlt
