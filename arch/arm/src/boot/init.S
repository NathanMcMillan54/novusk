.section .text._start

.extern setup
.globl _start
_start:
    bl setup

