ARCH =?
TARGET_ARCH =?
TARGET = arch/$(ARCH)/src/boot/$(TARGET_ARCH)-novusk.json

all: clean
	@ cargo build --target=$(TARGET)
	@ $(MAKE) -C arch/$(ARCH)/ link TARGET_ARCH=$(TARGET_ARCH)

clean:
	@ cargo clean
