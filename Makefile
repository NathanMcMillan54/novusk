ARCH =?
CC = gcc
LD = ld
LINKER_SCRIPT = arch/$(ARCH)/src/boot/linker.ld
TARGET = targets/$(ARCH)-novusk.json

all: cargo link

cargo:
	@ cargo clean
	@ cargo build --target=$(TARGET)

link:
	@ cp -r target/$(ARCH)-novusk/debug/libnovusk.a build/libnovusk.a
	$(MAKE) -C arch/$(ARCH)/ link

clean:
	@ rm -rf build/
	@ rm -rf arch/x86_64/src/boot/*.o
	@ rm -rf arch/x86_64/src/boot/bzImage

