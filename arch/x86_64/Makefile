TARGET = ../../targets/x86_64-novusk.json
AS = x86_64-linux-gnu-as
LD = x86_64-linux-gnu-ld

arch: boot
	@ echo "Compiling x86_64 kernel..."
	@ cargo build --target $(TARGET)

boot:
	@ echo "Compiling boot..."
	@ $(MAKE) -C ../../drivers/boot/nkuefi ARCH=x86_64

link:
	@ echo "Linking..."

all:
	@ echo "Test"
