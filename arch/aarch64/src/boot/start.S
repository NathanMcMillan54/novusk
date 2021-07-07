.globl _start

_start:
    bl die

die:
    wfe
    b die
