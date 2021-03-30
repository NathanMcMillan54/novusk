.code16

.extern main
.globl _start

_start:
    mov $boot_msg, %si
    mov $0xe, %ah
kernel_boot_message:
    lodsb
    cmp $0, %al
    je main
    int $0x10
    jmp kernel_boot_message

boot_msg:
    .asciz "Starting kernel...\n"

.fill 510-(.-_start), 1, 0
.word 0xaa55
