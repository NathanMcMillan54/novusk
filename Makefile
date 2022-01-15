ARCH =?
CONFIG =?
TARGET =?
LOCAL_TARGET =?
FEATURES =?
CRATE =?

ifeq ($(DEFCONFIG), True)
	CONFIG = kernel/konfig/$(ARCH)-defconfig.txt
endif

all: build_tools
	@ echo "Compiling $(ARCH) Novusk..."
	@ echo "Compiling based off CONFIG file ($(CONFIG))..."
	@ ./tools/build/buildkern/target/debug/buildkern $(CONFIG)

build_tools:
	@ echo "Compiling build tools..."
	@ $(MAKE) -C tools/build/buildkern all

package:
	@ cargo build --release -p $(CRATE) --target $(TARGET)

clean:
	@ cd tools/build/buildkern/ && cargo clean
