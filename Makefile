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

setup_build: dif build_tools setup
	@ $(link)

dif:
	@ cp -r arch/$(ARCH)/src/include/dif/$(DIF) arch/$(ARCH)/src/include/dif/kernel_dif.dif

build_tools:
	@ echo "Compiling build tools..."
	@ cargo build -p buildkern --target x86_64-unknown-linux-gnu

setup:
	@ rm -rf include/novusk/kms
	@ mkdir include/novusk/kms

buildkern:
	@ echo "Running buildkern with CONFIG file ($(CONFIG))..."
	@ ./target/x86_64-unknown-linux-gnu/debug/buildkern $(ARCH) $(CONFIG)
	@ echo "Finished compiling"

link:
	@ $(MAKE) -C arch/$(ARCH)/ KERNEL=$(KERNEL) PLATFORM=$(PLATFORM) link

clean:
	@ cargo clean
	@ cd arch/$(ARCH)/ && make clean
	@ cd include/novusk/ && make clean
