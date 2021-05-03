.section ".text.boot"

.extern setup

.globl _start
_start:
    bl setup
