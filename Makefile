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

all: dif build_tools setup build_config

dif:
	@ cp -r arch/$(ARCH)/src/include/dif/$(DIF) arch/$(ARCH)/src/include/dif/kernel_dif.dif

build_tools:
	@ echo "Compiling build tools..."
	@ cargo build -p buildkern

setup:
	@ rm -rf include/novusk/kms
	@ mkdir include/novusk/kms

build_config:
	@ echo "Compiling based off CONFIG file ($(CONFIG))..."
	@ ./target/debug/buildkern $(CONFIG)
	@ echo "Finished compiling"

clean:
	@ cargo clean
	@ cd arch/$(ARCH)/ && make clean
	@ cd include/novusk/ && make clean
