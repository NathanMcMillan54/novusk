BOOT = src/boot/
BZIMAGE = src/boot/bzImage
KERNEL = $(BOOT)kernel
LINKER_SCRIPT = src/boot/linker.ld

image:
	@ ld -o $(BZIMAGE) -T$(LINKER_SCRIPT) -o $(BZIMAGE) $(BOOT)header.o $(BOOT)multiboot.o $(KERNEL)
	@ echo "Created bzImage at arch/x86_64/src/boot/bzImage"
