ARCH =?
TARGET_ARCH =?

all: clean novusk link

novusk:
	@ echo "Compiling kernel..."
	@ $(MAKE) -C arch/$(ARCH)/ all ARCH=$(ARCH) TARGET_ARCH=$(TARGET_ARCH)

link:
	@ echo "Linking kernel..."
	@ $(MAKE) -C arch/$(ARCH)/ image

clean:
	@ rm -rf build/
	@ $(MAKE) -C arch/x86/ clean

