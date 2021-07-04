.section ".text.boot"

.extern LD_STACK_PTR

.globl _start
_start:
    ldr x30, =LD_STACK_PTR
    mov sp, x30

    bl virt_start
    wfe
