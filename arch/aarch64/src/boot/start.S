// Almost all arm32/64 devices have a different startup procces and needs a different startup script, this file just
// calls start then stops

.globl _start

_start:
    bl die

die:
    wfe
    b die
