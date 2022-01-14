ARCH =?
CONFIG =?
TARGET =?
FEATURES =?
CRATE =?

ifeq ($(ARCH), x86_64)
	CONFIG = kernel/konfig/x86_64-defconfig.txt
endif

all:
	@ cargo build --release --features $(FEATURES) --target $(TARGET)

package:
	@ cargo build --release -p $(CRATE) --target $(TARGET)

clean:
	@ cargo clean
