.section ".text.boot"

.extern initStart

.globl _start
    bl initStart
