.section ".text.boot"

.globl _start
.extern aarch64_boot_setup

    .org 0x80000
_start:
    ldr     x5, =_start
    mov     sp, x5

    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
3:  cbz     w6, 4f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 3b

4:  bl aarch64_boot_setup
    wfe
