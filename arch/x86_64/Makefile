BOOT_LOADER = 
COMPILER=x86_64-linux-gnu

all: $(BOOT_LOADER)

build:
	@ cargo build -p x86_64@0.1.0 --target ../../targets/x86_64-novusk.json --release

grub: build grub_boot grub_finish

grub_boot:
	@ rm -rf output/
	@ echo "Compiling Grub files..."
	@ mkdir output/
	@ $(COMPILER)-as -64 -nostdlib src/boot/multiboot.S -o output/multiboot.o
	@ $(COMPILER)-as -64 -nostdlib src/boot/multiboot_header.S -o output/multiboot_header.o
	@ echo "Done"

grub_finish:
	@ rm -rf ../../tools/grub/boot/novusk.bin
	@ rm -rf ../../tools/grub/boot/novusk.iso
	@ ld -n -T src/boot/multiboot.ld -o ../../tools/grub/boot/novusk.bin output/multiboot_header.o output/multiboot.o ../../target/x86_64-novusk/release/liblibx86_64_novusk.a
	@ grub-mkrescue -o ../../tools/grub/boot/novusk.iso ../../tools/grub/
