KERNEL =
KERNEL_IMG =?
KMS = ../../include/novusk/kms/
KMS_LIST = $(KMS)*.km
PLATFORM =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG = aadImage
endif

link: clean asm
	@ aarch64-linux-gnu-ld --allow-multiple-definition -Tsrc/boot/linker.ld src/boot/a53_start.o src/kernel/libaarch64.a src/kernel/irq/vector.o src/kernel/entry.o ../../target/aarch64-novusk/release/*.a -o src/boot/aadImage
	@ echo "Finished linking"

asm:
	@ mv ../../target/aarch64-novusk/release/libaarch64_kernel_lib.a src/kernel/libaarch64.a
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c src/boot/a53_start.S -o src/boot/a53_start.o
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c src/kernel/irq/vector.S -o src/kernel/irq/vector.o
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c src/kernel/entry.S -o src/kernel/entry.o

clean:
	@ rm -rf src/boot/aadImage
	@ rm -rf src/boot/*.img
	@ rm -rf src/boot/*.o
	@ rm -rf src/include/dif/kernel_dif.dif
	@ rm -rf src/kernel/irq/vector.o
	@ rm -rf src/kernel/libaarch64.a
