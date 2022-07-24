DEVICE_KERNEL=False
LINK_COMMAND =?
TARGET =?

ifeq ($(DEVICE_KERNEL), True)
	LINK_COMMAND = mv ../../target/$(TARGET)/release/a32dImage src/boot/a32dImage
endif

setup:
	@ echo "Setting up ARM for build..."

link:
	@ arm-none-eabi-objcopy --strip-all -O binary ../../target/$(TARGET)/release/a32dImage src/boot/a32dImage

clean:
	@ rm -rf src/boot/a32dImage
