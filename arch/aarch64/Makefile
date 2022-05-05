KERNEL =
KERNEL_IMG =?
KMS = ../../include/novusk/kms/
KMS_LIST = $(KMS)*.km
PLATFORM =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG = aadImage
endif

link: asm
	@ aarch64-linux-gnu-ld -Tsrc/boot/linker.ld ../arm/src/boot/start/a53_start.o ../../target/aarch64-novusk/release/*.a -o src/boot/aadImage
	@ echo "Finished linking"

asm:
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c ../arm/src/boot/start/a53_start.S -o ../arm/src/boot/start/a53_start.o

clean:
	@ cargo clean
	@ rm -rf src/boot/aadImage
	@ rm -rf src/kernel/cpu/irq/vector.o
	@ cd ../arm/ && make clean
