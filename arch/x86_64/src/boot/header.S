.code32

.globl _start
.extern bootloader_start_novusk

_start:
    cli
    call bootloader_start_novusk
    hlt
