#include "boot.h"

.globl _start

_start:
    ldr r4,=0xffffffff
    mov lr,r4

    bl boot_die
