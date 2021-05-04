.extern rpi_init
.globl _start
_start:
    bl rpi_init
    wfe
