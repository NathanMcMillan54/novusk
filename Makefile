ARCH =?
CONFIG =?
CONFIG_PATH = arch/$(ARCH)/src/configs/$(CONFIG)-config.txt
DEFCONFIG =?
KERNEL =
TARGET = targets/$(ARCH)-novusk.json
LOCAL_TARGET =?
FEATURES =?

ifeq ($(DEFCONFIG), True)
	CONFIG = arch/$(ARCH)/src/configs/$(ARCH)-defconfig.txt
endif

all: build_tools setup build_config build_arch
	@ sleep 1 && echo "Finished compiling Novusk"

build_tools:
	@ echo "Compiling build tools..."
	@ $(MAKE) -C tools/build/buildkern all

setup:
	@ rm -rf include/novusk/kms
	@ mkdir include/novusk/kms

build_config:
	@ echo "Compiling based off CONFIG file ($(CONFIG))..."
	@ ./tools/build/buildkern/target/debug/buildkern $(CONFIG)

build_arch:
	@ sleep 2
	@ echo "Compiling $(ARCH) Novusk..."
	@ sleep 1
	@ $(MAKE) -C arch/$(ARCH) all

clean:
	@ cd tools/build/buildkern/ && cargo clean
	@ cd drivers/boot/nkuefi && cargo clean
	@ cd drivers/gpu/vgag/ && cargo clean
	@ cd drivers/gpu/armfb/ && cargo clean
	@ cd drivers/platform/rpi && cargo clean
	@ cd arch/$(ARCH)/ && make clean
	@ cd include/novusk/ && make clean
