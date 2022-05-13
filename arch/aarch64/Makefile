KERNEL =
KERNEL_IMG =?
KMS = ../../include/novusk/kms/
KMS_LIST = $(KMS)*.km
PLATFORM =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG = aadImage
endif

link: clean asm
	@ aarch64-linux-gnu-ld -Tsrc/boot/linker.ld src/boot/kernel64.img ../../target/aarch64-novusk/release/*.a -o src/boot/aadImage
	@ echo "Finished linking"

asm:
	@ mv ../../target/aarch64-novusk/release/libaarch64_kernel_lib.a src/kernel/libaarch64.a
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c src/boot/a53_start.S -o src/boot/a53_start.o
	@ aarch64-linux-gnu-ld --relocatable src/boot/a53_start.o src/kernel/libaarch64.a -o src/boot/kernel64.img

clean:
	@ rm -rf src/boot/aadImage
	@ rm -rf src/boot/*.img
	@ rm -rf src/boot/*.o
	@ rm -rf src/kernel/cpu/irq/vector.o
	@ rm -rf src/kernel/libaarch64.a
