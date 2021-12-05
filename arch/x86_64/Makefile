TARGET_DIR = ../../target
AS = x86_64-linux-gnu-as
LD = x86_64-linux-gnu-ld

arch:
	@ $(AS) -o src/boot/header.o src/boot/header.S

link:
	@ $(LD) -o src/boot/bzImage --oformat binary -Ttext 0x7c00 src/boot/header.o $(TARGET_DIR)/x86_64-novusk/release/libarch_x86_64.rlib
