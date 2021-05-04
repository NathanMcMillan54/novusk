ARCH =?
TARGET_ARCH =?
BOARD =? virt

all:
	@ sh tools/build.sh $(TARGET_ARCH) $(BOARD)

image:
	@ echo "Creating kernel images..."
	@ sh tools/image.sh $(TARGET_ARCH)
	@ echo "Created Image at:"
	@ echo "arch/aarch64/src/boot/Image"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/Image

