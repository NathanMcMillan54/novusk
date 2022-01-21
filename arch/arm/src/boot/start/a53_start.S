#include "boot.h"

.section ".text.boot"

.global _start
.extern boot_setup

_start:
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f

1:  bl boot_die
2:

    ldr     x1, =_start
    mov     sp, x1

    bl      start_boot

    b       1b
