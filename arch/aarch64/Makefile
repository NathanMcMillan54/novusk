KERNEL =
KERNEL_IMG =?
KMS = ../../include/novusk/kms/
KMS_LIST = $(KMS)*.km
PLATFORM =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG = aadImage
endif

link: asm
	@ aarch64-linux-gnu-ld -Tsrc/boot/linker.ld src/boot/arm_kernel_boot.img ../../target/aarch64-novusk/release/*.a -o src/boot/aadImage
	@ echo "Finished linking"

asm:
	@ mv ../../target/aarch64-novusk/release/libarm.a src/kernel/libarm.a
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c ../arm/src/boot/start/a53_start.S -o ../arm/src/boot/start/a53_start.o
	@ aarch64-linux-gnu-ld --relocatable ../arm/src/boot/start/a53_start.o src/kernel/libarm.a -o src/boot/arm_kernel_boot.img

clean:
	@ cargo clean
	@ rm -rf src/boot/aadImage
	@ rm -rf src/kernel/cpu/irq/vector.o
	@ rm -rf src/kernel/libarm.a
	@ cd ../arm/ && make clean
