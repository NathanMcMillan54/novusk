ARCH =?
BOOT = src/boot/
ROOT = ../../
TARGET_ARCH =?

all:
	@ cargo build --target=targets/$(TARGET_ARCH)-novusk.json
	@ mv target/$(TARGET_ARCH)-novusk/debug/libx86.a $(ROOT)build/
	@ mv $(BOOT)header.o $(ROOT)build/

link: image

image:
	@ echo "Creating kernel image..."
	@ ld -o $(BOOT)bzImage --oformat binary -e _start -T$(BOOT)linker.ld -o $(BOOT)bzImage $(ROOT)build/kernel
	@ echo "Created bzImage"

clean:
	@ cargo clean
	@ rm -rf $(BOOT)bzImage
