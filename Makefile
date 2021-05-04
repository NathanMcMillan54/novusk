ARCH =?
TARGET_ARCH =?
DEVICE =?

all: clean build_tools novusk link

build_tools:
	@ echo "Compiling tools..."
	@ $(MAKE) -C tools/ all

novusk:
	@ echo "Compiling kernel..."
	@ $(MAKE) -C arch/$(ARCH)/ all ARCH=$(ARCH) TARGET_ARCH=$(TARGET_ARCH) BOARD=$(DEVICE)

link:
	@ mv tools/disk_img/target/debug/disk_img arch/$(ARCH)
	@ $(MAKE) -C arch/$(ARCH)/ image

clean:
	@ $(MAKE) -C arch/x86/ clean
	@ $(MAKE) -C arch/aarch64/ clean
	@ $(MAKE) -C tools/ clean
