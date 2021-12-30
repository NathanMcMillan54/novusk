TARGET = ../../targets/x86_64-novusk.json
AS = x86_64-linux-gnu-as
LD = x86_64-linux-gnu-ld

arch: boot
	@ cargo build --target $(TARGET)

boot:
	@ echo "Compiling boot..."
	@ cd ../../drivers/boot/nkuefi/
	@ make all ARCH=x86_64

link:
	@ echo "Linking..."
