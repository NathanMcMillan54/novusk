.globl _start
.extern binit

.org 0x4000

_start:
    bl binit
