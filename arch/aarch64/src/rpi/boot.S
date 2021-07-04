.section ".text.boot"

.globl _start

    .org 0x80000

_start:
    ldr     x5, =_start
    mov     sp, x5

    // Replace with arch/aarch64/src/rpi/memory::zer_bss_se
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
3:  cbz     w6, 4f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 3b

4:  bl rpi_setup
