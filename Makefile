ARCH =?
LINKER_SCRIPT = arch/$(ARCH)/src/boot/linker.ld
TARGET = targets/$(ARCH)-novusk.json

all: clean setup cargo link

setup:
	@ echo "Setting up Novusk..."
	@ mkdir build/

cargo:
	@ echo "Compiling kernel"
	@ cargo clean
	@ cargo build --target=$(TARGET)

link:
	@ echo "Linking kernel with external files..."
	@ cp -r target/$(ARCH)-novusk/debug/libnovusk.a build/libnovusk.a
	@ $(MAKE) -C arch/$(ARCH)/ link

clean:
	@ echo "Cleaning"
	@ rm -rf build/
	@ $(MAKE) -C arch/$(ARCH)/ clean
