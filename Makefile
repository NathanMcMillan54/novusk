ARCH =?
TARGET = arch/src/$(ARCH)/configs/$(ARCH)-novusk.json
CC =?
OS =?

all: uos kernel


uos:
	python3 arch/src/$(ARCH)/configs/build.py $(OS)


kernel: novusk image


novusk:
	cargo build --target=$(TARGET)


image:
	cargo bootimage --target $(TARGET)
	cp -r target/$(ARCH)-novusk/debug/bootimage-novusk.bin novusk

grub:
	mv target/x86-novusk/debug/bootimage-novusk.bin arch/src/x86/boot/iso/boot/
	cd arch/src/x86/boot/
	grub-mkrescue -o isonovusk.iso iso

qemu_x86:
	qemu-system-x86_64 -drive format=raw,file=novusk


clean:
	rm -rf novusk
	rm -rf target/
