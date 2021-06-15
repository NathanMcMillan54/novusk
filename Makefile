ARCH = x86_64
ARCH_FAM = x86
BOOT_METHOD = uefi
BOOT_TARGET = $(BOOT_METHOD)_boot
BUILD_TARGET = targets/$(ARCH)-novusk-$(BOOT_METHOD).json
DEVICE = default_machine

all:
	@ cargo build --features $(DEVICE),$(BOOT_TARGET) --target $(BUILD_TARGET)
	@ sh tools/image.sh $(ARCH) $(ARCH_FAM) $(BOOT_METHOD)
	@ echo "Created Novusk image:"
	@ echo "/Novusk"

clean:
	@ rm -rf Novusk
	@ cargo clean

docs:
	@ cargo doc
