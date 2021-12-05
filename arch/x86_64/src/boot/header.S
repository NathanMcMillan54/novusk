.code16

// Globals
.globl _start
.globl bios_print

// Externals
.extern main

_start:
    mov $start_msg, %si
    mov $0xe, %ah
    call bios_print

    call main
    call die

bios_print_ret:
    ret

bios_print:
    lodsb
    cmp $0, %al
    je bios_print_ret
    int $0x10
    jmp bios_print

die:
    hlt
    jmp die

// Text:
start_msg: .asciz "Starting kernel...\n"
die_msg: .asciz "Kernel died\n"

.fill 510-(.-_start), 1, 0
.word 0xaa55
