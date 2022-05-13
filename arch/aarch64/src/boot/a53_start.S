#include "boot.h"

.section ".text.boot"

.global _start
.extern aarch64_boot_setup
.extern boot_die

_start:
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbz x1, 2f

1:  bl boot_die
2:

    ldr x1, =_start
    mov sp, x1

    bl aarch64_boot_setup

    b 1b
