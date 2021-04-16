.code16

.extern entry32

.globl _start
.type _start @function

// Setup
_start:
    sti
    mov $kernel_msg, %si
    mov $0x0e, %ah

printKernelMsg:
    lodsb
    or %al, %al
    jz entry32
    int $0x10
    jmp printKernelMsg

kernel_msg:
    .asciz "Starting kernel\n\r"

