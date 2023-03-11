.code64
.globl _start
.extern start_main

_start:
    call start_main

    hlt
