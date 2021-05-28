// Main Virt boot file

.extern LD_STACK_PTR
.extern virt_init
.extern virt_kernel
.globl _start

.section ".text.boot"

_start:
    ldr x30, =LD_STACK_PTR
    mov sp, x30
    bl virt_init
    bl virt_kernel
    wfe
