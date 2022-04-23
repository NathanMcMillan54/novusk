.globl inb
.globl outb

inb:
	ldr w0,[x0]
	ret

outb:
    str w1,[x0]
    ret
