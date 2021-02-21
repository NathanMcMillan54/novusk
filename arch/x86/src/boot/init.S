    .code32
    .text

.extern initStart
.globl _start

_start:
    call initStart

.org 510
.word 0xaa55
