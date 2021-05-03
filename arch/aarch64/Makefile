ARCH =?
TARGET_ARCH =?

all:
	@ cargo build --target targets/$(TARGET_ARCH)-novusk.json -Z build-std=core -Z build-std-features=compiler-builtins-mem

image:
	@ echo "Creating kernel images..."
	@ sh tools/image.sh $(TARGET_ARCH)
	@ echo "Created Image at:"
	@ echo "arch/aarch64/src/boot/Image"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/Image

