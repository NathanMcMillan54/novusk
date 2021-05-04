.extern LD_STACK_PTR
.extern setup
.extern virt_init
.globl _start

.section ".text.boot"

_start:
    ldr x30, =LD_STACK_PTR
    mov sp, x30
    bl virt_init
    bl setup
    wfe
