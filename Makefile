ARCH =? x86
CC =?
OS =? none
TARGET = arch/$(ARCH)/src/configs/$(ARCH)-novusk.json

all: uos kernel

uos:
	python3 arch/$(ARCH)/src/configs/build.py $(OS)

kernel:
	cargo build --target=$(TARGET)

image:
	python3 arch/$(ARCH)/src/boot/image.py

qemu_aarch64:
	qemu-system-aarch64 -machine $(BOARD) -m 1024M -cpu $(CPU) -nographic -kernel novusk

qemu_x86:
	qemu-system-x86_64 -drive format=raw,file=novusk

clean:
	rm -rf novusk
	rm -rf target/
