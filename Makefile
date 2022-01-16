ARCH =?
CONFIG =?
CONFIG_PATH = arch/$(ARCH)/src/configs/$(CONFIG)-config.txt
DEFCONFIG =?
TARGET =?
LOCAL_TARGET =?
FEATURES =?
CRATE =?

ifeq ($(DEFCONFIG), True)
	CONFIG = arch/$(ARCH)/src/configs/$(ARCH)-defconfig.txt
endif

all: build_tools
	@ echo "Compiling $(ARCH) Novusk..."
	@ echo "Compiling based off CONFIG file ($(CONFIG))..."
	@ ./tools/build/buildkern/target/debug/buildkern $(CONFIG)
	@ $(MAKE) -C arch/$(ARCH) all

build_tools:
	@ echo "Compiling build tools..."
	@ $(MAKE) -C tools/build/buildkern all

package:
	@ cargo build --release -p $(CRATE) --target $(TARGET)

clean:
	@ cd tools/build/buildkern/ && cargo clean
	@ cd drivers/gpu/vgag/ && cargo clean
	@ cd arch/$(ARCH)/ && cargo clean
