ARCH =?
TARGET = targets/$(ARCH)-novusk.json

all: kernel image

kernel:
	@ cargo clean
	@ cargo build --target=$(TARGET)
	@ mv target/$(ARCH)-novusk/debug/libnovusk.a arch/$(ARCH)/src/boot/kernel

image:
	@ $(MAKE) -C arch/$(ARCH)/ image



