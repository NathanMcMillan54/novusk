.code16
.globl _start
_start
    mov $boot_msg, %si
    mov $0xe, %ah
print_char:
    lodsb
    cmp $0, %al
    je main
    int $0x10
    jmp print_char

boot_msg:
    .asciz "Starting kernel...\n"

.fill 510-(.-_start), 1, 0
.word 0xaa55
