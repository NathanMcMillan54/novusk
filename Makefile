ARCH =?
TARGET = targets/$(ARCH)-novusk.json

all: clean kernel image

kernel:
	@ $(MAKE) -C arch/$(ARCH)/ boot_files
	@ cargo clean
	@ cargo build --target=$(TARGET)
	@ mv target/$(ARCH)-novusk/debug/libnovusk.a arch/$(ARCH)/src/boot/kernel

image:
	@ $(MAKE) -C arch/$(ARCH)/ image

iso:
	@ sh tools/iso.sh

clean:
	@ cargo clean
	@ sh tools/clean/main.sh
