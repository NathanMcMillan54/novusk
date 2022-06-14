DEVICE_KERNEL=False
LINK_COMMAND =?
TARGET =?

ifeq ($(DEVICE_KERNEL), True)
	LINK_COMMAND = mv ../../target/$(TARGET)/release/a32dImage src/boot/a32dImage
endif

link:
	@ $(LINK_COMMAND)

clean:
	@ rm -rf src/boot/a32dImage
