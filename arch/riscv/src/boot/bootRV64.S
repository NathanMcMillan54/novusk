.globl _start

.option norvc

.section .data

.section .text.init

_start:
	csrr	t0, mhartid
	bnez	t0, 3f
	csrw	satp, zero

.option push
.option norelax
	la		gp, _global_pointer
.option pop
	la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:
	la		sp, _stack
	li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
	csrw	mstatus, t0

	la		t1, start_riscv_kernel
	csrw	mepc, t1

	la		t2, trap_vector
	csrw	mtvec, t2

	li		t3, (1 << 3) | (1 << 7) | (1 << 11)
	csrw	mie, t3
	la		ra, 4f
	mret
3:

4:
	wfi
	j		4b

