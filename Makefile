ARCH =? x86_64
DEVICE =?
TARGET = targets/$(ARCH)-novusk.json

all: clean novusk boot

novusk:
	@ echo "Compiling Novusk for $(ARCH)"
	@ $(MAKE) -C arch/$(ARCH) kernel BOARD=$(DEVICE)

boot:
	@ echo "Making bootable file of Novusk for $(ARCH)"
	@ $(MAKE) -C arch/$(ARCH)/ boot BOARD=$(DEVICE)

clean:
	@ cargo clean
	@ sh tools/clean/main.sh
