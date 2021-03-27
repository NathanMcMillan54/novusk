.code16

.extern kernel_init
.globl _start

_start:
	call kernel_init
	hlt

.fill 510-(.-_start), 1, 0
.word 0xaa55
