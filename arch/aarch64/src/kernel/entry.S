#include "entry.h"

.globl irq_vector_init
.globl vectors

    // Adds an entry to the vectors
	.macro kernel_ventry eln ht size label
	.align	7
	b	\label\eln\ht\size
	.endm

	//

	.macro kernel_entry
	sub	sp, sp, #S_FRAME_SIZE
	stp	x0, x1, [sp, #16 * 0]
	stp	x2, x3, [sp, #16 * 1]
	stp	x4, x5, [sp, #16 * 2]
	stp	x6, x7, [sp, #16 * 3]
	stp	x8, x9, [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]
	str	x30, [sp, #16 * 15]
	.endm

	.macro kernel_exit
	ldp	x0, x1, [sp, #16 * 0]
	ldp	x2, x3, [sp, #16 * 1]
	ldp	x4, x5, [sp, #16 * 2]
	ldp	x6, x7, [sp, #16 * 3]
	ldp	x8, x9, [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]
	ldr	x30, [sp, #16 * 15]
	add	sp, sp, #S_FRAME_SIZE
	eret
	.endm

// Vectors
.align 11

vectors:
    // EL1t
    kernel_ventry 1, t, 64, sync
    kernel_ventry 1, t, 64, irq
    kernel_ventry 1, t, 64, fiq
    kernel_ventry 1, t, 64, error

    // EL1h
    kernel_ventry 1, h, 64, sync
    kernel_ventry 1, h, 64, irq
    kernel_ventry 1, h, 64, fiq
    kernel_ventry 1, h, 64, error

    // EL0 64
    kernel_ventry 0, t, 64, sync
    kernel_ventry 0, t, 64, irq
    kernel_ventry 0, t, 64, fiq
    kernel_ventry 0, t, 64, error

    // EL0 32
    kernel_ventry 0, t, 32, sync
    kernel_ventry 0, t, 32, irq
    kernel_ventry 0, t, 32, fiq
    kernel_ventry 0, t, 32, error

el1_irq:
    kernel_entry
    // TODO: Call the main irq handler
    kernel_exit

// Initializes the IRQ vector
irq_vector_init:
    adr	x0, vectors
    msr	vbar_el1, x0
    ret

