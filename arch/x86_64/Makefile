BOOT = src/boot/
BZIMAGE = src/boot/bzImage
KERNEL = $(BOOT)kernel
LINKER_SCRIPT = src/boot/linker.ld

image:
	@ ld -o $(BZIMAGE) --oformat binary -e _start -T$(LINKER_SCRIPT) -o $(BZIMAGE) $(BOOT)header.o $(BOOT)kentry32.o $(KERNEL)
	@ echo "Created bzImage at arch/x86_64/src/boot/bzImage"

install:
	@ curl 'https://raw.githubusercontent.com/cirosantilli/x86-bare-metal-examples/master/common.h' > src/boot/boot.h
