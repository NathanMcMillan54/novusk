    .code32
    .text

.extern initStart
.globl _start

_start:
    call initStart
