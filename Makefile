MAJOR_VERSION = 4
MINOR_VERSION = 0
REALLY_MINOR_VERSION = 0
VERSION_NAME = Rust

ARCH =?
TARGET = targets/$(ARCH)-novusk.json

all: build_arch link
	@ echo "Compiling..."

build_arch:
	@ cd arch/$(ARCH)/ && make arch

link:
	@ echo "Linking..."
	@ cd arch/$(ARCH) && make link
