.global _start
.extern _bootstart

.section .text
.code32

_start:
    mov $stack_top, %esp

    call _bootstart
    hlt

check_multiboot:
    cmp $0x36d76289, %eax
    jne loop_error
    ret

loop_error:
    hlt
    jmp loop_error

stack_top:
