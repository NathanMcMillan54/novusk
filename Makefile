ARCH = x86
BOARD = virt
CC =?
OS = none
TARGET = arch/$(ARCH)/src/configs/$(ARCH)-novusk.json

all: build kernel

build:
	python3 arch/$(ARCH)/src/configs/build.py $(OS) $(BOARD)

kernel:
	cargo build --target=$(TARGET)

image:
	python3 arch/$(ARCH)/src/boot/image.py

qemu_aarch64:
	qemu-system-aarch64 -machine $(BOARD) -m 1024M -cpu $(CPU) -kernel novusk

qemu_x86:
	qemu-system-x86_64 -drive format=raw,file=novusk

clean:
	rm -rf novusk
	rm -rf target/
