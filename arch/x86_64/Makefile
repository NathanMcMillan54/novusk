BOOT = src/boot/
BZIMAGE = src/boot/bzImage
KERNEL = $(BOOT)kernel
LINKER_SCRIPT = src/boot/linker.ld

NASM = nasm -f elf64

boot_files:
	@ $(NASM) $(BOOT)header.asm
	@ $(NASM) $(BOOT)multiboot.asm
	@ $(NASM) $(BOOT)kernelEntry.asm

image:
	@ ld -n -o $(BZIMAGE) -T $(LINKER_SCRIPT) $(BOOT)multiboot.o $(BOOT)header.o $(BOOT)kernelEntry.o $(KERNEL)
	@ echo "Created bzImage at arch/x86_64/src/boot/bzImage"

clean:
	@ rm -rf $(BOOT)*.o
	@ rm -rf $(KERNEL)
	@ rm -rf $(BOOT)bzImage
