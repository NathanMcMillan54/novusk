// Almost all Aarch64 boards boot differently, for now this will only boot for RPi3/4

.section ".text.boot"

.globl _start

.org 0x80000

_start:
    ldr x5, =_start
    mov sp, x5
    bl clear_bss
    bl setup
    wfe
