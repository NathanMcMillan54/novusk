ARCH =?
TARGET_ARCH =?
BOARD =? virt

all:
	@ cargo build --target=targets/$(TARGET_ARCH)-novusk.json --features $(BOARD)

image:
	@ echo "Creating kernel images..."
	@ novus_boot target/aarch64-novusk/debug/aarch64.efi aarch64
	@ cp -r target/aarch64-novusk/debug/aarch64.fat src/boot/Image
	@ echo "Created Image at:"
	@ echo "arch/aarch64/src/boot/Image"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/Image

