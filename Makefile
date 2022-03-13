ARCH =?
CONFIG =?
CONFIG_PATH = arch/$(ARCH)/src/configs/$(CONFIG)-config.txt
DEFCONFIG = True
DIF = None
KERNEL = Image
PLATFORM = default
TARGET = targets/$(ARCH)-novusk.json
LOCAL_TARGET =?
FEATURES =?
SOC = unknown_soc

ifeq ($(DEFCONFIG), True)
	CONFIG = arch/$(ARCH)/src/configs/$(ARCH)-defconfig.txt
endif

ifeq ($(DEFCONFIG), False)
	CONFIG = arch/$(ARCH)/src/configs/$(ARCH)-$(PLATFORM)-defconfig.txt
endif

ifeq ($(DIF), None)
	DIF = empty_unknown.dif
endif

all: dif build_tools setup build_config build_arch
	@ sleep 1 && echo "Finished compiling Novusk"

dif:
	@ cp -r arch/$(ARCH)/src/include/dif/$(DIF) arch/$(ARCH)/src/include/dif/kernel_dif.dif

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
	@ $(MAKE) -C arch/$(ARCH) all KERNEL=$(KERNEL) PLATFORM=$(PLATFORM) SOC=$(SOC) DIF=$(DIF)

clean:
	@ cd tools/build/buildkern/ && cargo clean
	@ cd drivers/boot/nkuefi && cargo clean
	@ cd drivers/gpu/vgag/ && cargo clean
	@ cd drivers/gpu/armfb/ && cargo clean
	@ cd drivers/platform/rpi && cargo clean
	@ cd arch/$(ARCH)/ && make clean
	@ cd include/novusk/ && make clean
