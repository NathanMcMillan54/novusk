build:
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -static -c src/kernel/entry.S -o build/entry.o
	@ aarch64-linux-gnu-ar rcs build/libentry.a build/entry.o

link:
	@ cp -r ../../target/aarch64-novusk/release/libnovusk.a build/libnovusk.a
	@ aarch64-linux-gnu-ld -Tsrc/boot/linker.ld build/libnovusk.a build/entry.o -o src/boot/aadImage
	@ echo "Finished linking"
