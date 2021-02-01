ARCH =?
TARGET = arch/src/$(ARCH)/configs/$(ARCH)-novusk.json
CC =?

all: kernel image

kernel:
	cargo build --target=$(TARGET)

image:
	cargo bootimage --target $(TARGET)
	mv target/$(ARCH)-novusk/debug/bootimage-novusk.bin novusk

qemu_x86:
	qemu-system-x86_64 -drive format=raw,file=novusk

clean:
	rm -rf novusk
	rm -rf target/
	$(MAKE) arch/make clean
