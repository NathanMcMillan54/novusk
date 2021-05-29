ARCH =?
TARGET_ARCH =?
DEVICE =?

all: clean novusk link

novusk:
	@ echo "Compiling kernel..."
	@ $(MAKE) -C arch/$(ARCH)/ all ARCH=$(ARCH) TARGET_ARCH=$(TARGET_ARCH) BOARD=$(DEVICE)

link:
	@ $(MAKE) -C arch/$(ARCH)/ image

install:
	@ $(MAKE) -C tools/ install

clean:
	@ $(MAKE) -C arch/x86/ clean
	@ $(MAKE) -C arch/aarch64/ clean
	@ $(MAKE) -C tools/ clean
