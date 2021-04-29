ARCH =?
TARGET_ARCH =?

all: clean setup novusk link

setup:
	@ echo "Setting up Novusk for compiling..."
	@ mkdir build

novusk:
	@ echo "Compiling kernel..."
	@ $(MAKE) -C arch/$(ARCH)/ all ARCH=$(ARCH) TARGET_ARCH=$(TARGET_ARCH)

link:
	@ echo "Linking kernel..."
	@ cd build/ && ld -relocatable *.o *.a -o kernel
	@ $(MAKE) -C arch/$(ARCH)/ link
	@ $(MAKE) -C arch/$(ARCH)/ image

clean:
	@ rm -rf build/
	@ $(MAKE) -C arch/x86/ clean

