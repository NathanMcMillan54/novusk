.code16

.extern main
.globl _start
.type _start @function

_start:
    sti
    mov $kernel_msg, %si
    mov $0x0e, %ah
    call printKernelMsg

printKernelMsg:
    lodsb
    or %al, %al
    jz kernelEntry
    int $0x10
    jmp printKernelMsg

kernel_msg:
    .asciz "Starting kernel...\n\r"

kernelEntry:
    cld
    cli
    call main

