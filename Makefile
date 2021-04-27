ARCH =?
TARGET_ARCH =?
TARGET = arch/$(ARCH)/src/boot/$(TARGET_ARCH)-novusk.json

all:
	@ cargo build --target=$(TARGET)
