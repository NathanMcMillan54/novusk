BOOT = src/boot/
TARGET_ARCH =?

all:
	@ cargo build --target=targets/$(TARGET_ARCH)-novusk.json

image:
	@ mv target/$(TARGET_ARCH)-novusk/debug/x86.efi src/boot/bzImage

clean:
	@ cargo clean
	@ rm -rf $(BOOT)bzImage
