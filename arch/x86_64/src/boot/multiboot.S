.global _start
.extern _bootstart

.section .text
.code32

_start:
    mov $stack_top, %esp

    pushl %ebx
    pushl %eax
    call _bootstart
    hlt

loop_error:
    hlt
    jmp loop_error

.section .bss

stack_bottom:
    .skip 64
stack_top:

.section .rodata
