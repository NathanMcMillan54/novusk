ARCH =?
CC = gcc
LD = ld
LINKER_SCRIPT = arch/$(ARCH)/src/boot/linker.ld
TARGET = targets/$(ARCH)-novusk.json

all:
	@ mkdir build/
	@ cargo build --target=$(TARGET)
	@ mv target/$(ARCH)-novusk/debug/libnovusk.a build/

bootloader:
	@ cd arch/$(ARCH)/ && make boot
	@ cd arch/$(ARCH)/src/boot/ && mv init.o ../../../../build/

link:
	@ $(CC) -o novusk -ffreestanding -nostdlib build/init.o build/libnovusk.a

image:
	@ mv novusk iso/boot/ && grub-mkrescue -o novusk.iso iso

clean:
	@ rm -rf build/
	@ rm -rf target/
