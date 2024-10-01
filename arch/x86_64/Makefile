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
	@ $(COMPILER)-as -64 -nostdlib src/boot/grub.S -o output/grub.o
	@ $(COMPILER)-as -64 -nostdlib src/boot/grub_multiboot.S -o output/grub_multiboot.o
	@ echo "Done"

grub_finish:
	@ rm -rf ../../tools/grub/boot/novusk.bin
	@ rm -rf ../../tools/grub/boot/novusk.iso
	@ ld -n -T src/boot/grub.ld -o ../../tools/grub/boot/novusk.bin output/grub_multiboot.o output/grub.o ../../target/x86_64-novusk/release/liblibx86_64_novusk.a
	@ grub-mkrescue -o ../../tools/grub/boot/novusk.iso ../../tools/grub/
