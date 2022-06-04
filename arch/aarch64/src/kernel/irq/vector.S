.globl vectors
.globl irq_vector_init

    // This macro makes an entry in the vector table
	.macro	entry	label
	.align	7
	b	\label
	.endm

irq_vector_init:
    adr	x0, vectors		// load VBAR_EL1 with virtual
    msr	vbar_el1, x0		// vector table address
    ret

// Vector table
.align	11
vectors:
	entry	sync_invalid_el1t			// Synchronous EL1t
	entry	irq_invalid_el1t			// IRQ EL1t
	entry	fiq_invalid_el1t			// FIQ EL1t
	entry	error_invalid_el1t			// Error EL1t

	entry	sync_invalid_el1h			// Synchronous EL1h
	entry	el1_irq					// IRQ EL1h
	entry	fiq_invalid_el1h			// FIQ EL1h
	entry	error_invalid_el1h			// Error EL1h

	entry	sync_invalid_el0_64			// Synchronous 64-bit EL0
	entry	irq_invalid_el0_64			// IRQ 64-bit EL0
	entry	fiq_invalid_el0_64			// FIQ 64-bit EL0
	entry	error_invalid_el0_64			// Error 64-bit EL0

	entry	sync_invalid_el0_32			// Synchronous 32-bit EL0
	entry	irq_invalid_el0_32			// IRQ 32-bit EL0
	entry	fiq_invalid_el0_32			// FIQ 32-bit EL0
	entry	error_invalid_el0_32			// Error 32-bit EL0

sync_invalid_el1t:
    ret

irq_invalid_el1t:
    ret

fiq_invalid_el1t:
    ret

error_invalid_el1t:
    ret

sync_invalid_el1h:
    ret

el1_irq:
    ret

fiq_invalid_el1h:
    ret

error_invalid_el1h:
    ret

sync_invalid_el0_64:
    ret

irq_invalid_el0_64:
    ret

fiq_invalid_el0_64:
    ret

error_invalid_el0_64:
    ret

sync_invalid_el0_32:
    ret

irq_invalid_el0_32:
    ret

fiq_invalid_el0_32:
    ret

error_invalid_el0_32:
    ret
