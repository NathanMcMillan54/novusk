BOOT = src/boot/
KERNEL = src/kernel/
BOOT_METHOD =
ROOT = ../../

build:
	aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -static -c $(KERNEL)entry.S -o $(KERNEL)entry.o

link:
	@ aarch64-linux-gnu-ld -T$(BOOT)linker.ld $(ROOT)target/aarch64-novusk/release/libaarch64.a $(KERNEL)entry.o -o $(BOOT)kernel.bin
	@ aarch64-linux-gnu-objcopy $(BOOT)kernel.bin $(BOOT)kernel.elf
	@ echo "Finished linking"
