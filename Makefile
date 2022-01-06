MAJOR_VERSION = 4
MINOR_VERSION = 0
REALLY_MINOR_VERSION = 0
VERSION_NAME = Rust

ARCH =?
TARGET = targets/$(ARCH)-novusk.json

all: build_arch link
	@ echo "Compiling..."

build_arch:
	@ $(MAKE) -C arch/$(ARCH)/ arch

link:
	@ $(MAKE) -C arch/$(ARCH)/ link
