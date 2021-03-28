move:
	@ cp -r src/boot/header.o ../../build/header.o

image:
	@ ld -o src/boot/bzImage --oformat binary -e _start Ttext 0x7c00 -o src/boot/bzImage ../../build/header.o ../../build/libnovusk.a
	@ echo "Created bzImage at arch/x86_64/src/boot/bzImage"

clean:
	@ rm -rf src/boot/*.o
	@ rm -rf src/boot/bzImage
