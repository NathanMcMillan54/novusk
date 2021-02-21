ARCH = x86
BOARD = virt
CC =?
OS = none
TARGET = arch/$(ARCH)/src/configs/$(ARCH)-novusk.json

all: clean build kernel

build:
	@ python3 arch/$(ARCH)/src/configs/build.py $(OS) $(BOARD)

kernel:
	@ cargo build --target=$(TARGET)

image:
	@ python3 arch/$(ARCH)/src/boot/image.py

qemu_aarch64:
	qemu-system-aarch64 -machine $(BOARD) -m 1024M -cpu $(CPU) -nographic -kernel novusk

qemu_x86:
	qemu-system-x86_64 novusk

clean:
	@ cargo clean
	@ rm -rf novusk
