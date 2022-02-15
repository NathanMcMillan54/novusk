.globl main
.extern arm_boot_main

main:
    bl arm_boot_main
    bl boot_die

boot_die:
    wfe
    b .
