.code16

.globl unsafe_restart

unsafe_restart:
    ljmpw $0xf000, $0xfff0
