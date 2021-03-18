    .code16

.globl _start

_start:
    call kernel_init

.org 510
.word 0xaa55
