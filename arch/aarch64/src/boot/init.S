.section .text

.extern setup

.globl _start
_start:
    bl setup
