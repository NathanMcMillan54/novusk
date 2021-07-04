// Almost all Aarch64 boards boot differently, for now this will only boot for RPi3/4

.section ".text.boot"

.globl _start

_start:
    wfe
