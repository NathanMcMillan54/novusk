ROOT = ../../
BOOT = src/boot/
BOOT_METHOD =
BUILD_BOOT =
KERNEL = src/kernel/
LINK_COMMAND =

ifeq ($(BOOT_METHOD), uefi)
	BUILD_BOOT = cd ../../drivers/nkuefi && make bin TARGET=x86_64-unknown-uefi.json
	LINK_COMMAND = x86_64-linux-gnu-gcc -o $(BOOT)kernel.bin -Bstatic $(ROOT)target/x86_64-novusk/release/libx86_64.a
else ifeq ($(BOOT_METHOD), bios)
	BUILD_BOOT = x86_64-linux-gnu-as $(BOOT)boot.S -o $(BOOT)boot.o && x86_64-linux-gnu-as $(BOOT)start.S -o $(BOOT)start.o
	LINK_COMMAND = x86_64-linux-gnu-ld -T$(BOOT)linker.ld $(BOOT)boot.o $(BOOT)start.o $(ROOT)target/x86_64-novusk/release/libx86_64.a -o $(BOOT)kernel.bin
endif

build:

link: boot
	@ $(LINK_COMMAND)
	@ echo "Linked boot files to kernel library file (arch/x86_64/src/boot/kernel.bin)"

boot:
	@ $(BUILD_BOOT)
